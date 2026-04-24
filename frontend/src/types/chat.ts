export interface Message {
  id: number;
  sender: 'agent' | 'user';
  text: string;
  proactive?: boolean;
  timestamp?: string;
}
