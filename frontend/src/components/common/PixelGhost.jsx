import React from 'react';

const PixelGhost = ({ size = "w-16 h-16", animate = true }) => {
  return (
    <div className={`relative mx-auto mb-4 ${animate ? 'animate-pulse' : ''}`}>
      <img 
        src="/ghost.png"
        alt="Game Soul Ghost"
        className={`${size} pixel-art mx-auto`}
        style={{
          imageRendering: 'pixelated',
          filter: 'drop-shadow(0 0 10px rgba(255, 255, 255, 0.3))'
        }}
      />
      
      {/* Efecto de brillo opcional */}
      {animate && (
        <div className="absolute inset-0 animate-ping opacity-20 pointer-events-none">
          <div className={`${size} bg-white/30 rounded-lg mx-auto`} />
        </div>
      )}
    </div>
  );
};

export default PixelGhost;
