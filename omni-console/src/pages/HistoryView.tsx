export default function HistoryView() {
  const events = [
    { id: '1', time: 'Just now', repo: 'backend-api', file: 'architecture-overview.md', status: 'Success' },
    { id: '2', time: '10 mins ago', repo: 'frontend-app', file: 'components.md', status: 'Success' },
    { id: '3', time: '2 hours ago', repo: 'backend-api', file: 'api-contracts.md', status: 'Failed' },
  ];

  return (
    <div className="animate-fade-in">
      <div style={{ marginBottom: '2rem' }}>
        <h2 style={{ margin: '0 0 8px 0', fontSize: '1.5rem', fontWeight: '600' }}>Webhook Sync History</h2>
        <p style={{ margin: 0, color: 'var(--text-secondary)' }}>Live trace of SQLite ingested webhook events from GitLab pushing to Wiki.js.</p>
      </div>

      <div style={{
        backgroundColor: 'var(--bg-surface)',
        border: '1px solid var(--border-color)',
        borderRadius: '8px',
        overflow: 'hidden'
      }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left' }}>
          <thead style={{ backgroundColor: 'var(--bg-surface-hover)' }}>
            <tr>
              <th style={{ padding: '16px', fontWeight: '500', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border-color)' }}>Timestamp</th>
              <th style={{ padding: '16px', fontWeight: '500', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border-color)' }}>Repository</th>
              <th style={{ padding: '16px', fontWeight: '500', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border-color)' }}>Target File</th>
              <th style={{ padding: '16px', fontWeight: '500', color: 'var(--text-secondary)', borderBottom: '1px solid var(--border-color)' }}>Status</th>
            </tr>
          </thead>
          <tbody>
            {events.map(ev => (
              <tr key={ev.id} style={{ borderBottom: '1px solid var(--border-color)' }}>
                <td style={{ padding: '16px' }}>{ev.time}</td>
                <td style={{ padding: '16px', color: 'var(--accent-primary)' }}>{ev.repo}</td>
                <td style={{ padding: '16px' }}>{ev.file}</td>
                <td style={{ padding: '16px' }}>
                  <span style={{
                    padding: '4px 8px',
                    borderRadius: '4px',
                    fontSize: '0.85rem',
                    backgroundColor: ev.status === 'Success' ? 'rgba(82, 196, 26, 0.1)' : 'rgba(234, 96, 117, 0.1)',
                    color: ev.status === 'Success' ? 'var(--accent-success)' : 'var(--accent-danger)'
                  }}>
                    {ev.status}
                  </span>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
