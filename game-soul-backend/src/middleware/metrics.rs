use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use log::info;
use std::time::Instant;

// Estructura para el middleware de métricas
pub struct MetricsMiddleware;

// Implementación del factory para el middleware
impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MetricsMiddlewareService { service }))
    }
}

// Servicio que realiza el tracking de métricas
pub struct MetricsMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Capturar el tiempo de inicio
        let start_time = Instant::now();
        
        // Extraer información de la petición
        let method = req.method().clone();
        let path = req.path().to_owned();
        
        // Log de inicio de petición
        info!("Recibida petición: {} {}", method, path);
        
        // Ejecutar el servicio
        let fut = self.service.call(req);
        
        // Crear un future que mida el tiempo de respuesta
        Box::pin(async move {
            // Esperar el resultado
            let res = fut.await?;
            
            // Calcular el tiempo de respuesta
            let duration = start_time.elapsed();
            
            // Registrar la métrica
            info!(
                "Completada petición: {} {} - Status: {} - Tiempo: {:.2?}",
                method,
                path,
                res.status().as_u16(),
                duration
            );
            
            // TODO: Registrar en Prometheus cuando se implemente
            // prometheus::register_counter!("api_requests_total", "Total de peticiones API").inc();
            
            // Devolver la respuesta
            Ok(res)
        })
    }
}

// Factory para crear el middleware de métricas
pub fn metrics_middleware() -> MetricsMiddleware {
    MetricsMiddleware
}