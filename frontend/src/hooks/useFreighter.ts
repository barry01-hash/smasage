import { useState, useCallback, useEffect } from "react";
import { useWallet } from "../app/components/WalletContext";

/**
 * Custom hook to manage Freighter wallet interactions.
 * Encapsulates connection logic, installation checks, and state management.
 */
export function useFreighter() {
  const { publicKey, setPublicKey } = useWallet();
  const [isInstalled, setIsInstalled] = useState<boolean>(false);
  const [showInstallModal, setShowInstallModal] = useState(false);

  // Check for Freighter on mount
  useEffect(() => {
    setIsInstalled(!!window.freighterApi);
  }, []);

  /**
   * Triggers the Freighter connection flow.
   * If not installed, shows the installation modal.
   */
  const connect = useCallback(async () => {
    if (!window.freighterApi) {
      setShowInstallModal(true);
      return null;
    }

    try {
      const key = await window.freighterApi.getPublicKey();
      setPublicKey(key);
      return key;
    } catch (error) {
      console.error("[useFreighter] Connection failed:", error);
      return null;
    }
  }, [setPublicKey]);

  /**
   * Disconnects the wallet by clearing the public key from global context.
   */
  const disconnect = useCallback(() => {
    setPublicKey(null);
  }, [setPublicKey]);

  return {
    publicKey,
    isConnected: !!publicKey,
    isInstalled,
    showInstallModal,
    setShowInstallModal,
    connect,
    disconnect,
  };
}
