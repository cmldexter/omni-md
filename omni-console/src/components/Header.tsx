interface HeaderProps {
  title: string;
}

export default function Header({ title }: HeaderProps) {
  return (
    <header style={{
      height: 'var(--header-height)',
      backgroundColor: 'var(--bg-app)',
      borderBottom: '1px solid var(--border-color)',
      display: 'flex',
      alignItems: 'center',
      padding: '0 2rem',
      justifyContent: 'space-between'
    }}>
      <h1 style={{ 
        margin: 0, 
        fontSize: '1.2rem', 
        fontWeight: '600',
        color: 'var(--text-primary)'
      }}>
        {title}
      </h1>
      
      <div style={{ display: 'flex', alignItems: 'center', gap: '1rem' }}>
        <div style={{ 
          display: 'flex', 
          alignItems: 'center', 
          gap: '8px',
          padding: '6px 12px',
          backgroundColor: 'var(--bg-surface)',
          borderRadius: '20px',
          fontSize: '0.85rem'
        }}>
          <span style={{ 
            width: '8px', 
            height: '8px', 
            borderRadius: '50%', 
            backgroundColor: 'var(--accent-success)' 
          }}></span>
          Worker Online
        </div>
      </div>
    </header>
  );
}
