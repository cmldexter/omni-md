interface SidebarProps {
  currentView: 'mappings' | 'history';
  setCurrentView: (view: 'mappings' | 'history') => void;
}

export default function Sidebar({ currentView, setCurrentView }: SidebarProps) {
  const menuItems = [
    { id: 'mappings', label: 'Document Routing', icon: '🛣️' },
    { id: 'history', label: 'Sync History', icon: '⏱️' }
  ];

  return (
    <div style={{
      width: 'var(--sidebar-width)',
      backgroundColor: 'var(--bg-surface)',
      borderRight: '1px solid var(--border-color)',
      display: 'flex',
      flexDirection: 'column'
    }}>
      <div style={{
        padding: '24px 20px',
        fontSize: '1.2rem',
        fontWeight: '700',
        color: 'var(--accent-primary)',
        letterSpacing: '0.5px',
        borderBottom: '1px solid var(--border-color)',
        marginBottom: '10px'
      }}>
        Omni Console
      </div>

      <nav style={{ flex: 1, padding: '10px 0' }}>
        {menuItems.map(item => {
          const isActive = currentView === item.id;
          return (
            <div 
              key={item.id}
              onClick={() => setCurrentView(item.id as any)}
              style={{
                padding: '12px 24px',
                margin: '4px 16px',
                borderRadius: '6px',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                gap: '12px',
                backgroundColor: isActive ? 'var(--bg-surface-active)' : 'transparent',
                color: isActive ? 'var(--accent-primary)' : 'var(--text-primary)',
                transition: 'all 0.2s ease'
              }}
            >
              <span>{item.icon}</span>
              <span style={{ fontWeight: isActive ? '600' : '500' }}>{item.label}</span>
            </div>
          );
        })}
      </nav>

      <div style={{ padding: '20px', fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
        md-relay-worker v1.0.0
      </div>
    </div>
  );
}
