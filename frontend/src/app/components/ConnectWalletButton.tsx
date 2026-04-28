import React from 'react';
import { Button } from './Button';

export interface ConnectWalletButtonProps {
  onClick: () => void;
  publicKey?: string;
  isConnecting?: boolean;
}

function truncatePublicKey(key: string) {
  if (!key) return '';
  return key.slice(0, 4) + '...' + key.slice(-4);
}

export const ConnectWalletButton: React.FC<ConnectWalletButtonProps> = ({
  onClick,
  publicKey,
  isConnecting = false
}) => {
  const ariaLabel = isConnecting
    ? 'Connecting wallet, please wait'
    : publicKey
    ? `Wallet connected: ${truncatePublicKey(publicKey)}`
    : 'Connect Stellar wallet';

  return (
    <Button
      onClick={onClick}
      className="connect-wallet-btn"
      variant="primary"
      isLoading={isConnecting}
      aria-label={ariaLabel}
      loadingLabel="Connecting wallet"
    >
      {publicKey ? truncatePublicKey(publicKey) : 'Connect Wallet'}
    </Button>
  );
};

