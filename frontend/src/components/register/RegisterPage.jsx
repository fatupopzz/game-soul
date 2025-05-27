import React, { useState } from 'react';
import Layout from '../common/Layout';
import PixelGhost from '../common/PixelGhost';
import Button from '../common/Button';

const RegisterPage = ({ onSubmit, onBack }) => {
  const [name, setName] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = (e) => {
    e.preventDefault();
    
    if (!name.trim()) {
      setError('¡Necesitas un nombre!');
      return;
    }
    
    if (name.length < 2) {
      setError('Mínimo 2 caracteres');
      return;
    }
    
    onSubmit(name.trim());
  };

  return (
    <Layout>
      <div className="flex items-center justify-center min-h-screen px-4">
        <div className="max-w-md mx-auto w-full">
          <div className="text-center mb-8">
            <PixelGhost size="w-16 h-16" />
            
            <div className="bg-white/10 backdrop-blur-sm rounded-2xl p-6 border border-white/10">
              <h2 className="text-3xl font-mono text-white mb-4 font-bold">
                ¡Hola, gamer!
              </h2>
              <p className="text-white/70 font-mono text-sm">
                ¿Cómo te llamas?
              </p>
            </div>
          </div>
          
          <form onSubmit={handleSubmit} className="space-y-6">
            <div className="bg-white/5 backdrop-blur-sm rounded-2xl p-6 border border-white/10">
              <input
                type="text"
                value={name}
                onChange={(e) => {
                  setName(e.target.value);
                  setError('');
                }}
                placeholder="Tu nombre aquí..."
                className="w-full bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white placeholder-white/50 font-mono focus:outline-none focus:ring-2 focus:ring-blue-400/50"
                maxLength={20}
                autoFocus
              />
              
              {error && (
                <p className="text-red-300 font-mono text-xs mt-2">
                  ⚠️ {error}
                </p>
              )}
            </div>
            
            <div className="flex space-x-3">
              <Button
                type="button"
                onClick={onBack}
                variant="secondary"
                className="flex-1"
              >
                ← Volver
              </Button>
              
              <Button
                type="submit"
                variant="gradient"
                disabled={!name.trim()}
                className="flex-2"
              >
                Continuar →
              </Button>
            </div>
          </form>
        </div>
      </div>
    </Layout>
  );
};

export default RegisterPage;
