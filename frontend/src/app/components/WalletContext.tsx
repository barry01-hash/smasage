"use client";
import { createContext, useContext, useState, ReactNode } from 'react';

interface WalletContextType {
  publicKey: string | null;
  setPublicKey: (key: string | null) => void;
}

const WalletContext = createContext<WalletContextType | undefined>(undefined);

export const WalletProvider = ({ children }: { children: ReactNode }) => {
  const [publicKey, setPublicKey] = useState<string | null>(null);
  return (
    <WalletContext.Provider value={{ publicKey, setPublicKey }}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const ctx = useContext(WalletContext);
  if (!ctx) throw new Error('useWallet must be used within WalletProvider');
  return ctx;
};
