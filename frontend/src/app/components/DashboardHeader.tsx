import React from 'react';
import { WsStatusIndicator } from './WsStatusIndicator';

interface DashboardHeaderProps {
  children?: React.ReactNode;
  wsConnected?: boolean;
}

export const DashboardHeader: React.FC<DashboardHeaderProps> = ({ children, wsConnected = false }) => (
  <header className="dashboard-header">
    <div className="header-content">
      <div className="logo-section">
        <div className="brand">
          <span className="brand-name">Smasage</span>
          <div className="status-pill">
            <WsStatusIndicator connected={wsConnected} />
            <span className={`status-text ${wsConnected ? 'live' : 'connecting'}`}>
              {wsConnected ? 'Live' : 'Connecting'}
            </span>
          </div>
        </div>
      </div>
      <div className="header-actions">{children}</div>
    </div>
  </header>
);
