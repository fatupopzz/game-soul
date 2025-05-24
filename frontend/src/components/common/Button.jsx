import React from 'react';

const Button = ({ 
  children, 
  onClick, 
  variant = 'primary', 
  size = 'md',
  disabled = false,
  className = '',
  type = 'button',
  ...props 
}) => {
  const baseClasses = "font-mono transition-all duration-300 rounded-xl border backdrop-blur-sm";
  
  const variants = {
    primary: "bg-white/10 hover:bg-white/20 border-white/20 text-white hover:scale-105",
    secondary: "bg-white/5 hover:bg-white/10 border-white/20 text-white/70",
    gradient: "bg-gradient-to-r from-blue-500/20 to-purple-500/20 hover:from-blue-500/30 hover:to-purple-500/30 border-white/20 text-white hover:scale-105"
  };
  
  const sizes = {
    sm: "px-4 py-2 text-sm",
    md: "px-6 py-3 text-base",
    lg: "px-8 py-4 text-lg"
  };
  
  const classes = `
    ${baseClasses}
    ${variants[variant]}
    ${sizes[size]}
    ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
    ${className}
  `.trim();

  return (
    <button
      type={type}
      onClick={onClick}
      disabled={disabled}
      className={classes}
      {...props}
    >
      {children}
    </button>
  );
};

export default Button;
