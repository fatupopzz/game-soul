import React from 'react';
import Layout from '../common/Layout';
import PixelGhost from '../common/PixelGhost';
import Button from '../common/Button';

const LandingPage = ({ onStart }) => {
  return (
    <Layout>
      <div className="flex items-center justify-center min-h-screen px-4">
        <div className="text-center max-w-2xl mx-auto">
          {/* Logo/Mascota */}
          <PixelGhost size="w-20 h-20" animate={true} />
          
          {/* Título principal */}
          <div className="bg-white/20 backdrop-blur-sm rounded-2xl p-8 mb-8 border border-white/10 shadow-2xl">
            <h1 className="text-4xl md:text-6xl font-mono text-white mb-4 tracking-wider font-bold">
              BIENVENIDO A
            </h1>
            <h1 className="text-5xl md:text-7xl font-mono text-white mb-2 tracking-wider font-bold bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
              GAME SOUL
            </h1>
          </div>
          
          {/* Botón principal */}
          <Button 
            onClick={onStart}
            variant="gradient"
            size="lg"
            className="mb-6 min-w-64"
          >
            Descubre tu juego ideal
          </Button>
          
          {/* Descripción */}
          <p className="text-white/60 font-mono text-sm md:text-base leading-relaxed max-w-md mx-auto">
            Sistema inteligente de recomendaciones basado en tu estado emocional :D
          </p>
        </div>
      </div>
    </Layout>
  );
};

export default LandingPage;