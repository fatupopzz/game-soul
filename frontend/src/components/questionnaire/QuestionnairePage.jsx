import React, { useState } from 'react';
import Layout from '../common/Layout';
import PixelGhost from '../common/PixelGhost';
import Button from '../common/Button';

const QuestionnairePage = ({ userName, onSubmit, onBack }) => {
  const [currentQuestion, setCurrentQuestion] = useState(0);
  const [answers, setAnswers] = useState({});
  const [isLoading, setIsLoading] = useState(false);

  // Preguntas que mapean exactamente a tu backend
  const questions = [
    {
      id: 'tipo_experiencia',
      question: '¿Qué tipo de experiencia buscas?',
      options: [
        { 
          value: 'relajante', 
          label: 'Algo relajante y tranquilo', 
          emoji: '🧘‍♀️',
          description: 'Quiero desestresarme y relajarme'
        },
        { 
          value: 'emocion', 
          label: 'Una experiencia emocionante', 
          emoji: '🎢',
          description: 'Busco aventura y emoción'
        },
        { 
          value: 'desafio', 
          label: 'Un desafío que me pruebe', 
          emoji: '⚔️',
          description: 'Quiero superar obstáculos difíciles'
        },
        { 
          value: 'exploracion', 
          label: 'Explorar y descubrir', 
          emoji: '🗺️',
          description: 'Me gusta investigar mundos nuevos'
        },
        { 
          value: 'conexion', 
          label: 'Conectar con otros jugadores', 
          emoji: '👥',
          description: 'Prefiero experiencias sociales'
        }
      ]
    },
    {
      id: 'estado_animo',
      question: '¿Cómo te sientes ahora mismo?',
      options: [
        { 
          value: 'tranquilo', 
          label: 'Tranquilo y relajado', 
          emoji: '😌',
          description: 'En paz, sin prisa'
        },
        { 
          value: 'energico', 
          label: 'Con energía y ganas de acción', 
          emoji: '⚡',
          description: 'Listo para la acción'
        },
        { 
          value: 'curioso', 
          label: 'Curioso y explorador', 
          emoji: '🤔',
          description: 'Quiero aprender algo nuevo'
        },
        { 
          value: 'nostalgico', 
          label: 'Nostálgico y reflexivo', 
          emoji: '🌅',
          description: 'Con ganas de recordar y reflexionar'
        },
        { 
          value: 'estresado', 
          label: 'Estresado, necesito despejarme', 
          emoji: '😤',
          description: 'Necesito olvidarme de todo'
        }
      ]
    },
    {
      id: 'actividad_preferida',
      question: '¿Qué actividad te llama más la atención?',
      options: [
        { 
          value: 'construir', 
          label: 'Construir y crear cosas', 
          emoji: '🏗️',
          description: 'Me gusta dar forma a mis ideas'
        },
        { 
          value: 'competir', 
          label: 'Competir contra otros', 
          emoji: '🏆',
          description: 'Quiero demostrar mis habilidades'
        },
        { 
          value: 'descubrir', 
          label: 'Descubrir secretos y misterios', 
          emoji: '🔍',
          description: 'Me encanta resolver enigmas'
        },
        { 
          value: 'historia', 
          label: 'Vivir una gran historia', 
          emoji: '📚',
          description: 'Quiero una narrativa envolvente'
        }
      ]
    },
    {
      id: 'tiempo_disponible',
      question: '¿Cuánto tiempo tienes para jugar?',
      options: [
        { 
          value: 'corto', 
          label: '30 minutos o menos', 
          emoji: '⏰',
          description: 'Una sesión rápida'
        },
        { 
          value: 'medio', 
          label: '1-2 horas', 
          emoji: '🕐',
          description: 'Una buena sesión'
        },
        { 
          value: 'largo', 
          label: '3+ horas', 
          emoji: '🌙',
          description: 'Tengo toda la tarde/noche'
        }
      ]
    },
    {
      id: 'meta_emocional',
      question: '¿Qué quieres sentir después de jugar?',
      options: [
        { 
          value: 'calma', 
          label: 'Calma y paz interior', 
          emoji: '☮️',
          description: 'Relajado y en armonía'
        },
        { 
          value: 'satisfaccion', 
          label: 'Satisfacción por lograr algo', 
          emoji: '💪',
          description: 'Orgulloso de mis logros'
        },
        { 
          value: 'asombro', 
          label: 'Asombro y maravilla', 
          emoji: '🤯',
          description: 'Sorprendido por algo increíble'
        },
        { 
          value: 'diversion', 
          label: 'Diversión pura', 
          emoji: '😂',
          description: 'Haberme reído y divertido'
        },
        { 
          value: 'conexion', 
          label: 'Conexión emocional profunda', 
          emoji: '💝',
          description: 'Tocado por una historia o personaje'
        }
      ]
    }
  ];

  const currentQuestionData = questions[currentQuestion];
  const isLastQuestion = currentQuestion === questions.length - 1;

  const handleAnswer = (questionId, answer) => {
    const newAnswers = { ...answers, [questionId]: answer };
    setAnswers(newAnswers);
    
    // Auto-avanzar después de seleccionar
    setTimeout(() => {
      if (isLastQuestion) {
        submitQuestionnaire(newAnswers);
      } else {
        setCurrentQuestion(currentQuestion + 1);
      }
    }, 300);
  };

  const submitQuestionnaire = async (finalAnswers) => {
    setIsLoading(true);
    
    try {
      const response = await fetch('http://localhost:8080/api/questionnaire', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          user_id: `${userName.toLowerCase().replace(/\s+/g, '_')}_${Date.now()}`,
          user_name: userName,
          answers: finalAnswers
        })
      });

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data = await response.json();
      console.log('✅ Cuestionario enviado exitosamente:', data);
      
      onSubmit({
        userName,
        answers: finalAnswers,
        backendResponse: data
      });
      
    } catch (error) {
      console.error('❌ Error enviando cuestionario:', error);
      
      // Fallback con datos simulados si el backend falla
      onSubmit({
        userName,
        answers: finalAnswers,
        backendResponse: {
          status: 'error',
          message: 'Backend no disponible - usando datos de prueba',
          recommendations: [
            { 
              id: 'fallback-1', 
              name: 'Stardew Valley', 
              description: 'Juego relajante perfecto para ti',
              matchScore: 0.85,
              reasons: ['Muy relajante', 'Creativo']
            },
            { 
              id: 'fallback-2', 
              name: 'Minecraft', 
              description: 'Ideal para crear y explorar',
              matchScore: 0.78,
              reasons: ['Creativo', 'Exploración']
            }
          ]
        }
      });
    } finally {
      setIsLoading(false);
    }
  };

  const handleBack = () => {
    if (currentQuestion > 0) {
      setCurrentQuestion(currentQuestion - 1);
    } else {
      onBack();
    }
  };

  if (isLoading) {
    return (
      <Layout>
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center">
            <PixelGhost size="w-20 h-20" animate={true} />
            <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-8 border border-white/10 mt-6">
              <h2 className="text-2xl font-mono text-white mb-4">
                Analizando el alma de {userName}...
              </h2>
              <div className="flex justify-center space-x-2 text-2xl animate-pulse">
                <span>🔮</span>
                <span>✨</span>
                <span>🎮</span>
              </div>
              <p className="text-white/60 font-mono text-sm mt-4">
                Conectando con el sistema de recomendaciones...
              </p>
            </div>
          </div>
        </div>
      </Layout>
    );
  }

  return (
    <Layout>
      <div className="flex items-center justify-center min-h-screen px-4">
        <div className="max-w-2xl mx-auto w-full">
          {/* Header */}
          <div className="text-center mb-8">
            <PixelGhost size="w-16 h-16" />
            
            <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 border border-white/10">
              <div className="text-white/60 font-mono text-sm mb-2">
                ¡Hola {userName}! • Pregunta {currentQuestion + 1} de {questions.length}
              </div>
              <h2 className="text-2xl md:text-3xl font-mono text-white mb-4 font-bold">
                {currentQuestionData.question}
              </h2>
            </div>
          </div>
          
          {/* Opciones */}
          <div className="space-y-4 mb-8">
            {currentQuestionData.options.map((option) => {
              const isSelected = answers[currentQuestionData.id] === option.value;
              
              return (
                <button
                  key={option.value}
                  onClick={() => handleAnswer(currentQuestionData.id, option.value)}
                  className={`w-full text-left p-4 rounded-xl border transition-all duration-300 hover:scale-105 group ${
                    isSelected 
                      ? 'bg-blue-500/20 border-blue-400/50 shadow-lg shadow-blue-500/20' 
                      : 'bg-white/5 hover:bg-white/10 border-white/10 hover:border-white/20'
                  }`}
                >
                  <div className="flex items-start space-x-4">
                    <span className={`text-3xl transition-transform group-hover:scale-110 ${
                      isSelected ? 'animate-bounce' : ''
                    }`}>
                      {option.emoji}
                    </span>
                    <div className="flex-1">
                      <h3 className="text-white font-mono font-medium mb-1">
                        {option.label}
                      </h3>
                      <p className="text-white/60 font-mono text-sm">
                        {option.description}
                      </p>
                    </div>
                    {isSelected && (
                      <div className="text-blue-400 text-xl">
                        ✓
                      </div>
                    )}
                  </div>
                </button>
              );
            })}
          </div>
          
          {/* Navegación */}
          <div className="flex justify-between items-center">
            <Button
              onClick={handleBack}
              variant="secondary"
              size="md"
            >
              ← {currentQuestion > 0 ? 'Anterior' : 'Volver'}
            </Button>
            
            {/* Barra de progreso */}
            <div className="flex-1 mx-6">
              <div className="bg-white/10 rounded-full h-2 overflow-hidden">
                <div 
                  className="bg-gradient-to-r from-blue-400 to-purple-400 h-full transition-all duration-500"
                  style={{ width: `${((currentQuestion + 1) / questions.length) * 100}%` }}
                />
              </div>
              <div className="text-center mt-2">
                <span className="text-white/60 font-mono text-xs">
                  {Math.round(((currentQuestion + 1) / questions.length) * 100)}% completado
                </span>
              </div>
            </div>
            
            <div className="w-20"></div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default QuestionnairePage;
