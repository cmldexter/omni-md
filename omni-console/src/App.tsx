import { useState } from 'react';
import Sidebar from './components/Sidebar';
import Header from './components/Header';
import MappingsView from './pages/MappingsView';
import HistoryView from './pages/HistoryView';
import './index.css';

function App() {
  const [currentView, setCurrentView] = useState<'mappings' | 'history'>('mappings');

  return (
    <div style={{ display: 'flex', height: '100vh', width: '100vw' }}>
      <Sidebar currentView={currentView} setCurrentView={setCurrentView} />
      
      <div style={{ flex: 1, display: 'flex', flexDirection: 'column' }}>
        <Header title={currentView === 'mappings' ? "Document Mappings" : "Sync History"} />
        
        <main style={{ flex: 1, padding: '2rem', overflowY: 'auto' }}>
          {currentView === 'mappings' && <MappingsView />}
          {currentView === 'history' && <HistoryView />}
        </main>
      </div>
    </div>
  );
}

export default App;
