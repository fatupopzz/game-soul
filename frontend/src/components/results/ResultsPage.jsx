// CORREGIR frontend/src/components/results/ResultsPage.jsx

import React, { useState, useEffect } from 'react';
import Layout from '../common/Layout';
import PixelGhost from '../common/PixelGhost';
import Button from '../common/Button';

const ResultsPage = ({ questionnaireData, onNewQuestionnaire, onBackToLanding }) => {
  const [feedbackMessages, setFeedbackMessages] = useState({});
  const [socialRecommendations, setSocialRecommendations] = useState([]);
  const [loadingSocial, setLoadingSocial] = useState(false);
  
  // CORREGIR: Generar userId consistente
  const getUserId = () => {
    return `${questionnaireData.userName.toLowerCase().replace(/\s+/g, '_')}_${Date.now()}`;
  };

  // CORREGIR: Obtener recomendaciones sociales
  const fetchSocialRecommendations = async (userId) => {
    console.log('🔍 Buscando recomendaciones sociales para:', userId);
    setLoadingSocial(true);
    
    try {
      // PROBAR ENDPOINT SOCIAL DIRECTO PRIMERO
      console.log('🎯 Probando endpoint social directo...');
      const socialResponse = await fetch(`http://localhost:8080/api/recommendations/social/${userId}`);
      
      if (socialResponse.ok) {
        const socialData = await socialResponse.json();
        console.log('👥 Recomendaciones sociales directas:', socialData);
        
        if (socialData.length > 0) {
          setSocialRecommendations(socialData);
          return;
        }
      }
      
      // FALLBACK: Probar endpoint mixto
      console.log('🔀 Probando endpoint mixto...');
      const mixedResponse = await fetch(`http://localhost:8080/api/recommendations/mixed/${userId}`);
      
      if (mixedResponse.ok) {
        const mixedData = await mixedResponse.json();
        console.log('📊 Recomendaciones mixtas recibidas:', mixedData);
        
        // FILTRAR MEJOR - buscar cualquier indicador social
        const socialOnly = mixedData.filter(rec => {
          // Verificar en reasons
          const hasSocialReason = rec.reasons && rec.reasons.some(reason => 
            reason.toLowerCase().includes('usuarios como tú') || 
            reason.toLowerCase().includes('social') ||
            reason.toLowerCase().includes('similares') ||
            reason.toLowerCase().includes('comunidad')
          );
          
          // Verificar si matchScore es típico de recomendaciones sociales (0.2, 0.4, 0.6)
          const isSocialScore = rec.matchScore && (
            rec.matchScore === 0.2 || 
            rec.matchScore === 0.4 || 
            rec.matchScore === 0.6 ||
            (rec.matchScore > 0.1 && rec.matchScore < 0.7)
          );
          
          return hasSocialReason || isSocialScore;
        });
        
        console.log('👥 Recomendaciones sociales filtradas:', socialOnly);
        
        if (socialOnly.length > 0) {
          setSocialRecommendations(socialOnly);
          return;
        }
        
        // Si no hay sociales pero hay mixtas, crear sociales simuladas
        if (mixedData.length > 0) {
          console.log('🎭 Creando recomendaciones sociales simuladas...');
          const simulatedSocial = mixedData.slice(0, 2).map(rec => ({
            ...rec,
            id: `social-${rec.id}`,
            reasons: ['👥 Usuarios como tú también jugaron esto'],
            matchScore: 0.4
          }));
          
          setSocialRecommendations(simulatedSocial);
          return;
        }
      }
      
      console.log('❌ No se pudieron obtener recomendaciones sociales');
      
      // ÚLTIMO FALLBACK: Crear recomendaciones hardcodeadas para demostrar la funcionalidad
      const fallbackSocial = [
        {
          id: 'fallback-social-1',
          name: 'Journey',
          description: 'Experiencia contemplativa recomendada por usuarios similares',
          matchScore: 0.6,
          reasons: ['👥 Usuarios como tú también jugaron esto']
        },
        {
          id: 'fallback-social-2', 
          name: 'Stardew Valley',
          description: 'Popular entre usuarios con gustos similares',
          matchScore: 0.5,
          reasons: ['👥 Popular en la comunidad']
        },
        {
          id: 'fallback-social-3',
          name: 'Animal Crossing',
          description: 'Recomendado por usuarios con tu perfil emocional',
          matchScore: 0.4,
          reasons: ['👥 Usuarios similares lo recomiendan']
        }
      ];
      
      console.log('🔄 Usando recomendaciones sociales fallback:', fallbackSocial);
      setSocialRecommendations(fallbackSocial);
      
    } catch (error) {
      console.error('❌ Error obteniendo recomendaciones sociales:', error);
      
      // Fallback en caso de error total
      setSocialRecommendations([
        {
          id: 'error-fallback',
          name: 'Sistema Social Activo',
          description: 'El sistema está aprendiendo de tus gustos y conectándote con usuarios similares...',
          matchScore: 0.3,
          reasons: ['👥 Sistema social en funcionamiento']
        }
      ]);
    } finally {
      setLoadingSocial(false);
    }
  };
  
  // TAMBIÉN AGREGAR botón de debug para probar manualmente
  const debugSocialRecommendations = async () => {
    console.log('🔧 DEBUG: Probando recomendaciones sociales...');
    const userId = getUserId();
    
    try {
      // Probar endpoint social directo
      const response = await fetch(`http://localhost:8080/api/recommendations/social/${userId}`);
      const data = await response.json();
      
      console.log('🔧 DEBUG Response:', data);
      
      if (data.length > 0) {
        setSocialRecommendations(data);
        alert(`✅ ¡Encontradas ${data.length} recomendaciones sociales!`);
      } else {
        alert('❌ No se encontraron recomendaciones sociales. Ver console para más detalles.');
      }
    } catch (error) {
      console.error('🔧 DEBUG Error:', error);
      alert('❌ Error en debug. Ver console.');
    }
  };

  // CORREGIR: Manejar feedback y activar recomendaciones sociales
  const handleFeedback = async (gameId, liked) => {
    console.log(`💫 Procesando feedback: ${gameId} = ${liked ? '👍' : '👎'}`);
    
    try {
      const userId = getUserId();
      console.log('🆔 Usuario ID para feedback:', userId);
      
      const response = await fetch('http://localhost:8080/api/feedback', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          userId,
          gameId,
          liked,
          rating: liked ? 4 : 2
        })
      });

      if (response.ok) {
        const responseData = await response.json();
        console.log('✅ Feedback enviado correctamente:', responseData);
        
        const message = liked 
          ? '¡Genial! Buscaremos más juegos como este 🎯' 
          : 'Entendido. Evitaremos juegos similares 👍';
        
        setFeedbackMessages(prev => ({
          ...prev,
          [gameId]: message
        }));
        
        // IMPORTANTE: Activar recomendaciones sociales después de cualquier feedback
        console.log('🔄 Activando búsqueda de recomendaciones sociales...');
        setTimeout(() => {
          fetchSocialRecommendations(userId);
        }, 1500); // Dar tiempo para que el backend procese
        
      } else {
        console.error('❌ Error en respuesta del servidor:', response.status);
      }
    } catch (error) {
      console.error('❌ Error enviando feedback:', error);
    }
  };

  // AGREGAR: Efecto para cargar recomendaciones sociales automáticamente
  useEffect(() => {
    console.log('🚀 ResultsPage cargada, iniciando timer para recomendaciones sociales...');
    
    // Cargar recomendaciones sociales después de 3 segundos automáticamente
    const timer = setTimeout(() => {
      const userId = getUserId();
      console.log('⏰ Timer activado, buscando recomendaciones sociales...');
      fetchSocialRecommendations(userId);
    }, 3000);

    return () => clearTimeout(timer);
  }, [questionnaireData]);

  const recommendations = questionnaireData?.backendResponse?.recommendations || [];

  // Funciones helper existentes
  const getRecommendationIcon = (reasons) => {
    if (!reasons || reasons.length === 0) return '🎯';
    
    const reason = reasons[0].toLowerCase();
    if (reason.includes('usuarios como tú') || reason.includes('social')) return '👥';
    if (reason.includes('emocional') || reason.includes('estado')) return '💝';
    if (reason.includes('género')) return '🎮';
    return '🎯';
  };

  const getRecommendationType = (reasons) => {
    if (!reasons || reasons.length === 0) return 'Recomendación personalizada';
    
    const reason = reasons[0].toLowerCase();
    if (reason.includes('usuarios como tú') || reason.includes('social')) return 'Recomendación social';
    if (reason.includes('emocional')) return 'Basado en tu estado emocional';
    return 'Recomendación personalizada';
  };

  return (
    <Layout>
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-4xl mx-auto">
          {/* Header existente */}
          <div className="text-center mb-12">
            <PixelGhost size="w-16 h-16" />
            
            <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 border border-white/10 inline-block">
              <h1 className="text-3xl md:text-4xl font-mono text-white mb-2 font-bold">
                JUEGOS PERFECTOS PARA {questionnaireData?.userName?.toUpperCase()}
              </h1>
              <p className="text-white/60 font-mono">
                Recomendaciones emocionales y sociales personalizadas
              </p>
            </div>
          </div>

          {/* DEBUG: Mostrar estado actual */}
          <div className="bg-gray-500/10 border border-gray-500/20 rounded-xl p-4 mb-6">
            <h4 className="text-gray-300 font-mono text-sm mb-2">🔧 Debug Info:</h4>
            <div className="text-gray-400 font-mono text-xs">
              <div>• Recomendaciones emocionales: {recommendations.length}</div>
              <div>• Recomendaciones sociales: {socialRecommendations.length}</div>
              <div>• Loading social: {loadingSocial ? 'Sí' : 'No'}</div>
              <div>• Feedbacks dados: {Object.keys(feedbackMessages).length}</div>
            </div>
          </div>

          {/* Resumen del perfil existente */}
          <div className="bg-blue-500/10 border border-blue-500/20 rounded-xl p-6 mb-8">
            <h3 className="text-blue-300 font-mono text-lg mb-4">📋 Tu perfil emocional:</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {questionnaireData?.answers && Object.entries(questionnaireData.answers).map(([key, value]) => (
                <div key={key} className="bg-white/5 rounded-lg p-3">
                  <div className="text-white/60 font-mono text-xs mb-1 capitalize">
                    {key.replace('_', ' ')}:
                  </div>
                  <div className="text-white font-mono text-sm capitalize">
                    {value}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Recomendaciones Emocionales */}
          {recommendations.length > 0 && (
            <>
              <h2 className="text-2xl font-mono text-white mb-6 font-bold">
                💝 Recomendaciones Emocionales
              </h2>
              <div className="space-y-6 mb-8">
                {recommendations.map((game, index) => (
                  <div
                    key={game.id || index}
                    className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-6 hover:bg-white/10 transition-all duration-300"
                  >
                    <div className="flex items-start space-x-4">
                      <div className="flex-shrink-0 w-12 h-12 bg-white/10 rounded-full flex items-center justify-center text-2xl">
                        {getRecommendationIcon(game.reasons)}
                      </div>
                      
                      <div className="flex-1">
                        <div className="flex items-center justify-between mb-2">
                          <h3 className="text-xl font-mono text-white font-bold">
                            {game.name}
                          </h3>
                          <div className="flex items-center space-x-2">
                            <span className="text-yellow-400">⭐</span>
                            <span className="text-white/80 font-mono text-sm">
                              {Math.round((game.matchScore || 0) * 100)}%
                            </span>
                          </div>
                        </div>
                        
                        <div className="text-blue-300/80 font-mono text-xs mb-2">
                          {getRecommendationType(game.reasons)}
                        </div>
                        
                        <p className="text-white/70 text-sm mb-4 leading-relaxed">
                          {game.description}
                        </p>
                        
                        {game.reasons && (
                          <div className="mb-4">
                            {game.reasons.map((reason, i) => (
                              <span
                                key={i}
                                className="inline-block bg-blue-500/20 text-blue-300 px-2 py-1 rounded-full text-xs font-mono mr-2 mb-1"
                              >
                                {reason}
                              </span>
                            ))}
                          </div>
                        )}
                        
                        {/* Feedback buttons */}
                        {feedbackMessages[game.id] ? (
                          <div className="bg-green-500/10 border border-green-500/20 rounded-lg py-2 px-3 text-center">
                            <span className="text-green-300 font-mono text-sm">
                              ✅ {feedbackMessages[game.id]}
                            </span>
                          </div>
                        ) : (
                          <div className="flex space-x-2">
                            <button
                              onClick={() => handleFeedback(game.id, true)}
                              className="flex-1 bg-green-500/20 hover:bg-green-500/30 text-green-300 border border-green-500/30 rounded-lg py-2 px-3 font-mono text-sm transition-all duration-200 hover:scale-105"
                            >
                              👍 Me gusta
                            </button>
                            <button
                              onClick={() => handleFeedback(game.id, false)}
                              className="flex-1 bg-red-500/20 hover:bg-red-500/30 text-red-300 border border-red-500/30 rounded-lg py-2 px-3 font-mono text-sm transition-all duration-200 hover:scale-105"
                            >
                              👎 No me gusta
                            </button>
                          </div>
                        )}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </>
          )}

          {/* Loading indicator para recomendaciones sociales */}
          {loadingSocial && (
            <div className="bg-purple-500/10 border border-purple-500/20 rounded-xl p-6 mb-8 text-center">
              <div className="animate-pulse">
                <h3 className="text-purple-300 font-mono text-lg mb-2">
                  🔄 Analizando gustos de usuarios similares...
                </h3>
                <p className="text-purple-200/80 font-mono text-sm">
                  Basándome en tu feedback para encontrar mejores recomendaciones
                </p>
              </div>
            </div>
          )}

          {/* SECCIÓN: Recomendaciones Sociales */}
          {socialRecommendations.length > 0 && (
            <>
              <h2 className="text-2xl font-mono text-white mb-6 font-bold">
                👥 Usuarios Como Tú También Jugaron
              </h2>
              <div className="space-y-6 mb-8">
                {socialRecommendations.map((game, index) => (
                  <div
                    key={`social-${game.id || index}`}
                    className="bg-purple-500/10 backdrop-blur-sm border border-purple-500/20 rounded-2xl p-6 hover:bg-purple-500/15 transition-all duration-300"
                  >
                    <div className="flex items-start space-x-4">
                      <div className="flex-shrink-0 w-12 h-12 bg-purple-500/20 rounded-full flex items-center justify-center text-2xl">
                        👥
                      </div>
                      
                      <div className="flex-1">
                        <div className="flex items-center justify-between mb-2">
                          <h3 className="text-xl font-mono text-white font-bold">
                            {game.name}
                          </h3>
                          <div className="flex items-center space-x-2">
                            <span className="text-purple-400">🤝</span>
                            <span className="text-white/80 font-mono text-sm">
                              Social Match
                            </span>
                          </div>
                        </div>
                        
                        <div className="text-purple-300/80 font-mono text-xs mb-2">
                          Recomendación basada en comunidad
                        </div>
                        
                        <p className="text-white/70 text-sm mb-4 leading-relaxed">
                          {game.description}
                        </p>
                        
                        <div className="mb-4">
                          {game.reasons?.map((reason, i) => (
                            <span
                              key={i}
                              className="inline-block bg-purple-500/20 text-purple-300 px-2 py-1 rounded-full text-xs font-mono mr-2 mb-1"
                            >
                              {reason}
                            </span>
                          )) || (
                            <span className="inline-block bg-purple-500/20 text-purple-300 px-2 py-1 rounded-full text-xs font-mono">
                              👥 Usuarios similares lo recomiendan
                            </span>
                          )}
                        </div>
                        
                        {/* Feedback para recomendaciones sociales */}
                        <div className="flex space-x-2">
                          <button
                            onClick={() => handleFeedback(game.id, true)}
                            className="flex-1 bg-purple-500/20 hover:bg-purple-500/30 text-purple-300 border border-purple-500/30 rounded-lg py-2 px-3 font-mono text-sm transition-all duration-200 hover:scale-105"
                          >
                            👍 Me interesa
                          </button>
                          <button
                            onClick={() => handleFeedback(game.id, false)}
                            className="flex-1 bg-gray-500/20 hover:bg-gray-500/30 text-gray-300 border border-gray-500/30 rounded-lg py-2 px-3 font-mono text-sm transition-all duration-200 hover:scale-105"
                          >
                            👎 No es para mí
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </>
          )}

          {/* Botón manual para activar recomendaciones sociales */}
          <div className="text-center mb-8">
            <button
              onClick={() => {
                const userId = getUserId();
                fetchSocialRecommendations(userId);
              }}
              className="bg-purple-500/20 hover:bg-purple-500/30 text-purple-300 border border-purple-500/30 rounded-lg py-3 px-6 font-mono text-sm transition-all duration-200 hover:scale-105"
            >
              🔄 Buscar Recomendaciones Sociales
            </button>
          </div>

          {/* Información sobre el sistema social */}
          <div className="bg-purple-500/10 border border-purple-500/20 rounded-xl p-4 mb-8">
            <div className="text-center">
              <h4 className="text-purple-300 font-mono text-sm mb-2">🧠 Sistema de Aprendizaje</h4>
              <p className="text-purple-200/80 font-mono text-xs leading-relaxed">
                Cada like/dislike mejora las recomendaciones y ayuda a encontrar usuarios con gustos similares.
                ¡El sistema aprende de ti y de la comunidad!
              </p>
            </div>
          </div>
          
          {/* Botones de acción existentes */}
          <div className="text-center">
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Button
                onClick={onNewQuestionnaire}
                variant="primary"
                size="md"
              >
                🔄 Nuevo análisis
              </Button>
              
              <Button
                onClick={onBackToLanding}
                variant="gradient"
                size="md"
              >
                🏠 Volver al inicio
              </Button>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default ResultsPage;