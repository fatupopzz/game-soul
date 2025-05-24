import React from 'react';

const Layout = ({ children, className = "" }) => {
  return (
    <div className={`min-h-screen relative overflow-hidden ${className}`}>
      {/* El fondo de figma */}
      <div 
        className="fixed inset-0 w-full h-full bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900"
        style={{
          backgroundImage: `url('/landing page.svg')`,
          zIndex: -2,
          opacity: 1
        }}
      />
      {/* SVG DE BURBUJAS */}
      <div 
        className="fixed inset-0 w-full h-full bg-cover bg-center bg-no-repeat animate-pulse"
        style={{
          backgroundImage: `url('/Burbujas.svg')`,
          zIndex: -2,
          opacity: 0.3
        }}
      />
      
      {/* Contenido */}
      <div className="relative z-10">
        {children}
      </div>
    </div>
  );
};

export default Layout;
