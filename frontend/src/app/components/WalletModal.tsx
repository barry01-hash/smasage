import React, { useEffect } from 'react';
import { X } from 'lucide-react';

interface WalletModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const WalletModal: React.FC<WalletModalProps> = ({ isOpen, onClose }) => {
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    if (isOpen) {
      window.addEventListener('keydown', handleEscape);
      // Prevent scrolling when modal is open
      document.body.style.overflow = 'hidden';
    }

    return () => {
      window.removeEventListener('keydown', handleEscape);
      document.body.style.overflow = 'unset';
    };
  }, [isOpen, onClose]);

  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div 
        className="modal-content" 
        onClick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
      >
        <button 
          className="modal-close-icon" 
          onClick={onClose}
          aria-label="Close modal"
          tabIndex={0}
        >
          <X size={20} />
        </button>
        
        <h2 id="modal-title" style={{ fontSize: '1.75rem', marginBottom: '1rem' }}>
          Freighter Wallet Not Detected
        </h2>
        
        <p className="text-muted" style={{ marginBottom: '2rem' }}>
          Please install the Freighter browser extension to securely connect your Stellar wallet and start crushing your goals.
        </p>
        
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
          <a 
            href="https://freighter.app/" 
            target="_blank" 
            rel="noopener noreferrer" 
            className="install-link"
          >
            Install Freighter
          </a>
          
          <button 
            onClick={onClose} 
            className="close-modal"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
