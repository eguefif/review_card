import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

import CardCrafter from './components/create/CardCrafter';

function App() {
  return <CardCrafter />;
}

export default App;
