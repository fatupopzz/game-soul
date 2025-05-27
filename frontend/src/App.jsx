import React, { useState } from 'react';
import LandingPage from './components/landing/LandingPage';
import RegisterPage from './components/register/RegisterPage';
import QuestionnairePage from './components/questionnaire/QuestionnairePage';
import ResultsPage from './components/results/ResultsPage';

const App = () => {
  const [currentStep, setCurrentStep] = useState('landing'); // 'landing' | 'register' | 'questionnaire' | 'results'
  const [userName, setUserName] = useState('');
  const [questionnaireData, setQuestionnaireData] = useState(null);

  const handleStart = () => {
    setCurrentStep('register');
  };

  const handleRegisterSubmit = (name) => {
    setUserName(name);
    setCurrentStep('questionnaire');
  };

  const handleQuestionnaireSubmit = (data) => {
    console.log('✅ Cuestionario completado:', data);
    setQuestionnaireData(data);
    setCurrentStep('results');
  };

  const handleBackToLanding = () => {
    setCurrentStep('landing');
    setUserName('');
    setQuestionnaireData(null);
  };

  const handleBackToRegister = () => {
    setCurrentStep('register');
  };

  const handleNewQuestionnaire = () => {
    setCurrentStep('questionnaire');
    setQuestionnaireData(null);
  };

  return (
    <div className="font-mono">
      {currentStep === 'landing' && (
        <LandingPage onStart={handleStart} />
      )}
      
      {currentStep === 'register' && (
        <RegisterPage 
          onSubmit={handleRegisterSubmit}
          onBack={handleBackToLanding}
        />
      )}

      {currentStep === 'questionnaire' && (
        <QuestionnairePage
          userName={userName}
          onSubmit={handleQuestionnaireSubmit}
          onBack={handleBackToRegister}
        />
      )}

      {currentStep === 'results' && (
        <ResultsPage 
          questionnaireData={questionnaireData}
          onNewQuestionnaire={handleNewQuestionnaire}
          onBackToLanding={handleBackToLanding}
        />
      )}
    </div>
  );
};

export default App;
