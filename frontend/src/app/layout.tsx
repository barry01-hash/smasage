
import type { Metadata } from 'next';
import './globals.css';
import { WalletProvider } from './components/WalletContext';

export const metadata: Metadata = {
  title: 'Smasage | AI Portfolio Manager',
  description: 'AI Portfolio Manager natively on Stellar',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <head>
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700&display=swap" rel="stylesheet" />
      </head>
      <body>
        <WalletProvider>{children}</WalletProvider>
      </body>
    </html>
  );
}
