"use client"
import React, { useState } from 'react';
import { Target, Activity } from 'lucide-react';
import { PortfolioStats } from './components/PortfolioStats';
import { evaluateGoalStatus, getStatusColor, type GoalData } from '../utils/goalProjection';
import PortfolioChart from './PortfolioChart';
import { DashboardHeader } from './components/DashboardHeader';
import { ConnectWalletButton } from './components/ConnectWalletButton';
import { useWallet } from './components/WalletContext';
import { ErrorBoundary } from './components/ErrorBoundary';
import { useChat } from '../hooks/useChat';
import { ChatInterface } from './components/ChatInterface';

export default function Home() {
  const { publicKey, setPublicKey } = useWallet();
  const [showInstallModal, setShowInstallModal] = useState(false);
    // Connect Wallet logic
    const handleConnectWallet = async () => {
      if (!window.freighterApi) {
        setShowInstallModal(true);
        return;
      }
      try {
        const key = await window.freighterApi.getPublicKey();
        setPublicKey(key);
      } catch {
        // Optionally handle rejection
      }
    };

  // Goal data
  const goalData: GoalData = {
    currentBalance: 12450,
    targetAmount: 18000,
    targetDate: '2026-08-01',
    monthlyContribution: 500,
    expectedAPY: 8.5,
  };

  const { messages, inputState, setInputState, isTyping, handleSendMessage, allocations, wsConnected } = useChat({
    userId: 'user-demo-001',
    goalData,
  });
  const goalResult = evaluateGoalStatus(goalData);
  const progress = goalResult.progressPercentage;
  const goalStatus = goalResult.status;

  return (
    <ErrorBoundary fallbackMessage="The dashboard failed to load. Please try again.">
      <>
        <DashboardHeader wsConnected={wsConnected}>
          <ConnectWalletButton onClick={handleConnectWallet} publicKey={publicKey || undefined} />
        </DashboardHeader>
        {showInstallModal && (
          <div className="modal-overlay">
            <div className="modal-content">
              <h2>Freighter Wallet Not Detected</h2>
              <p>Please install the Freighter browser extension to connect your wallet.</p>
              <a href="https://freighter.app/" target="_blank" rel="noopener noreferrer" className="install-link">Install Freighter</a>
              <button onClick={() => setShowInstallModal(false)} className="close-modal">Close</button>
            </div>
          </div>
        )}
        <main className="app-container">
        {/* Left Panel - Dashboard */}
        <div className="glass-panel">
          <h1>Smasage Portfolio</h1>
          <p className="text-muted" style={{ marginBottom: '2.5rem' }}>
            Real-time on-chain tracking • Stellar Mainnet 🚀
          </p>

          <PortfolioStats
            totalValue={goalData.currentBalance}
            apy={goalData.expectedAPY}
            valueChange={12.4}
          />

          <div className="goal-section">
            <div className="goal-header">
              <div>
                <h3 style={{ fontSize: '1.25rem', marginBottom: '4px' }}>European Vacation</h3>
                <p className="text-muted" style={{ fontSize: '0.9rem' }}>Target: $18,000 by Aug 2026</p>
                <p style={{ fontSize: '0.85rem', color: getStatusColor(goalStatus), fontWeight: 600, marginTop: '4px' }}>
                  Status: {goalStatus}
                </p>
              </div>
              <Target size={32} color={getStatusColor(goalStatus)} opacity={0.8} />
            </div>

            <div className="progress-bar-container">
              <div className="progress-bar-fill" style={{ width: `${progress}%` }}></div>
            </div>
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.85rem', color: 'var(--text-muted)', fontWeight: 500 }}>
              <span>68% Completed</span>
              <span>$5,550 Remaining</span>
            </div>
          </div>

          <div className="allocation-list">
            <h3 style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '1.1rem', marginBottom: '1.25rem', marginTop: '1rem' }}>
              <Activity size={18} /> Active Strategy Routes
            </h3>

            <PortfolioChart
              allocations={allocations}
              width={320}
              height={280}
              showLegend={true}
              animated={true}
            />
          </div>
        </div>

        {/* Right Panel - Chat Agent */}
        <ChatInterface
          messages={messages}
          inputState={inputState}
          setInputState={setInputState}
          isTyping={isTyping}
          handleSendMessage={handleSendMessage}
        />
      </main>
      </>
    </ErrorBoundary>
  );
}
