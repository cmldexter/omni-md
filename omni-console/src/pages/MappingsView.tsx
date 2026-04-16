export default function MappingsView() {
  return (
    <div className="animate-fade-in">
      <div style={{ marginBottom: '2rem' }}>
        <h2 style={{ margin: '0 0 8px 0', fontSize: '1.5rem', fontWeight: '600' }}>Repository Configurations</h2>
        <p style={{ margin: 0, color: 'var(--text-secondary)' }}>Map your Backstage TechDocs directories to destination Wiki.js repositories.</p>
      </div>

      <div style={{
        backgroundColor: 'var(--bg-surface)',
        border: '1px solid var(--border-color)',
        borderRadius: '8px',
        padding: '24px',
        marginBottom: '24px'
      }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
          <div>
            <h3 style={{ margin: 0, color: 'var(--accent-secondary)' }}>backend-api</h3>
            <div style={{ fontSize: '0.85rem', color: 'var(--text-secondary)', marginTop: '4px' }}>
              https://gitlab.example.com/my-org/backend-api.git
            </div>
          </div>
          <button style={{
            backgroundColor: 'transparent',
            border: '1px solid var(--border-color)',
            color: 'var(--text-primary)',
            padding: '8px 16px',
            borderRadius: '4px',
            cursor: 'pointer'
          }}>Edit Mapping</button>
        </div>
        
        <table style={{ width: '100%', borderCollapse: 'collapse' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid var(--border-color)', textAlign: 'left', color: 'var(--text-secondary)' }}>
              <th style={{ padding: '8px', fontWeight: '500' }}>Source (docs/)</th>
              <th style={{ padding: '8px', fontWeight: '500' }}>Destination (Wiki)</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td style={{ padding: '12px 8px', borderBottom: '1px solid var(--border-color)' }}>architecture-overview.md</td>
              <td style={{ padding: '12px 8px', borderBottom: '1px solid var(--border-color)', color: 'var(--accent-primary)' }}>engineering/backend/architecture.md</td>
            </tr>
          </tbody>
        </table>
      </div>

      <button style={{
        backgroundColor: 'var(--accent-primary)',
        color: '#000',
        border: 'none',
        padding: '10px 20px',
        borderRadius: '4px',
        fontWeight: '600',
        cursor: 'pointer',
        boxShadow: '0 4px 6px rgba(121, 235, 210, 0.2)'
      }}>+ Add Repository Mapping</button>
    </div>
  );
}
