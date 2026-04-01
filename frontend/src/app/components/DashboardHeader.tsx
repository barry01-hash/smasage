import React from 'react';

export const DashboardHeader: React.FC<{ children?: React.ReactNode }> = ({ children }) => (
  <header className="dashboard-header" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '1rem 2rem', borderBottom: '1px solid #eee' }}>
    <div style={{ fontWeight: 700, fontSize: '1.5rem' }}>Smasage</div>
    <div>{children}</div>
  </header>
);
