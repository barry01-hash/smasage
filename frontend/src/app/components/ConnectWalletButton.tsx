import React from 'react';

export interface ConnectWalletButtonProps {
  onClick: () => void;
  publicKey?: string;
}

function truncatePublicKey(key: string) {
  if (!key) return '';
  return key.slice(0, 4) + '...' + key.slice(-4);
}

export const ConnectWalletButton: React.FC<ConnectWalletButtonProps> = ({ onClick, publicKey }) => {
  return (
    <button onClick={onClick} className="connect-wallet-btn">
      {publicKey ? truncatePublicKey(publicKey) : 'Connect Wallet'}
    </button>
  );
};
