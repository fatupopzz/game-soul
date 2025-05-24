import React, { useState } from 'react';
import Layout from '../common/Layout';
import PixelGhost from '../common/PixelGhost';
import Button from '../common/Button';

const ResultsPage = ({ questionnaireData, onNewQuestionnaire, onBackToLanding }) => {
  const [feedbackMessages, setFeedbackMessages] = useState({});

  const handleFeedback = async (gameId, liked) => {
    try {
      const userId = `${questionnaireData.userName.toLowerCase().replace(/\s+/g, '_')}_${Date.now()}`;
      
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
        const message = liked 
          ? '¡Genial! Buscaremos más juegos como este 🎯' 
          : 'Entendido. Evitaremos juegos similares 👍';
        
        setFeedbackMessages(prev => ({
          ...prev,
          [gameId]: message
        }));
        
        console.log('✅ Feedback enviado y sistema social actualizado');
      }
    } catch (error) {
      console.error('❌ Error enviando feedback:', error);
    }
  };

  const recommendations = questionnaireData?.backendResponse?.recommendations || [];

  // Función para determinar el ícono del tipo de recomendación
  const getRecommendationIcon = (reasons) => {
    if (!reasons || reasons.length === 0) return '🎯';
    
    const reason = reasons[0].toLowerCase();
    if (reason.includes('usuarios como tú')) return '👥';
    if (reason.includes('emocional') || reason.includes('estado')) return '💝';
    if (reason.includes('género')) return '🎮';
    return '🎯';
  };

  const getRecommendationType = (reasons) => {
    if (!reasons || reasons.length === 0) return 'Recomendación personalizada';
    
    const reason = reasons[0].toLowerCase();
    if (reason.includes('usuarios como tú')) return 'Recomendación social';
    if (reason.includes('emocional')) return 'Basado en tu estado emocional';
    return 'Recomendación personalizada';
  };

  return (
    <Layout>
      <div className="min-h-screen px-4 py-8">
        <div className="max-w-4xl mx-auto">
          {/* Header */}
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

          {/* Resumen del perfil */}
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

          {/* Recomendaciones */}
          {recommendations.length > 0 ? (
            <div className="space-y-6 mb-8">
              {recommendations.map((game, index) => (
                <div
                  key={game.id || index}
                  className="bg-white/5 backdrop-blur-sm border border-white/10 rounded-2xl p-6 hover:bg-white/10 transition-all duration-300"
                  style={{ animationDelay: `${index * 0.1}s` }}
                >
                  <div className="flex items-start space-x-4">
                    {/* Ícono del tipo de recomendación */}
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
                      
                      {/* Feedback */}
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
          ) : (
            <div className="bg-yellow-500/10 border border-yellow-500/20 rounded-xl p-6 mb-8 text-center">
              <h3 className="text-yellow-300 font-mono text-lg mb-2">⚠️ Sin recomendaciones</h3>
              <p className="text-yellow-200/80 font-mono text-sm">
                No se pudieron obtener recomendaciones del backend. ¡Pero tu perfil ha sido guardado!
              </p>
            </div>
          )}

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
          
          {/* Botones de acción */}
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
