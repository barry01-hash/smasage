import React, { useRef, useEffect } from 'react';
import { Bot, Send, CheckCircle2, AlertCircle } from 'lucide-react';
import type { Message } from '../../types/chat';

interface ChatInterfaceProps {
  messages: Message[];
  inputState: string;
  setInputState: (value: string) => void;
  isTyping: boolean;
  handleSendMessage: (e: React.FormEvent) => void;
"use client";

import React, { useEffect, useRef, useState } from "react";
import { AlertCircle, Bot, CheckCircle2, Send } from "lucide-react";

export interface ChatMessage {
  id: number;
  sender: "agent" | "user";
  text: string;
  proactive?: boolean;
  timestamp?: string;
}

export interface ChatInterfaceProps {
  messages: ChatMessage[];
  isTyping: boolean;
  onSendMessage: (message: string) => void;
  placeholder?: string;
}

export function ChatInterface({
  messages,
  inputState,
  setInputState,
  isTyping,
  handleSendMessage,
}: ChatInterfaceProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // Auto scroll
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, isTyping]);

  return (
    <div className="glass-panel">
      <div className="chat-container">
        <div className="chat-header">
          <div className="agent-avatar">
            <Bot size={28} />
          </div>
          <div>
            <h2 style={{ margin: 0, fontSize: '1.25rem' }}>OpenClaw Agent</h2>
            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '0.85rem', color: 'var(--success)' }}>
              <CheckCircle2 size={12} fill="var(--success)" color="var(--bg-card)" /> Online
            </div>
          </div>
        </div>

        <div className="chat-messages">
          {messages.map((msg) => (
            <div key={msg.id} className={`message ${msg.sender}`}>
              {msg.proactive && (
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '0.75rem', color: 'var(--accent-primary)', marginBottom: '4px', fontWeight: 600, textTransform: 'uppercase' }}>
                  <AlertCircle size={12} /> Proactive Nudge
                </div>
              )}
              <div className="message-bubble">{msg.text}</div>
            </div>
          ))}
          {isTyping && (
            <div className="message agent">
              <div className="typing-indicator">
                <span></span><span></span><span></span>
              </div>
            </div>
          )}
          <div ref={messagesEndRef} />
        </div>

        <form onSubmit={handleSendMessage} className="chat-input-container">
          <input
            type="text"
            placeholder="Ask Smasage to adjust goals..."
            value={inputState}
            onChange={(e) => setInputState(e.target.value)}
          />
          <button type="submit" className="send-button">
            <Send size={18} />
          </button>
        </form>
      </div>
    </div>
  );
}
  isTyping,
  onSendMessage,
  placeholder = "Ask Smasage to adjust goals...",
}: ChatInterfaceProps) {
  const [inputValue, setInputValue] = useState("");
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages, isTyping]);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const trimmed = inputValue.trim();
    if (!trimmed) return;

    onSendMessage(trimmed);
    setInputValue("");
  };

  return (
    <div className="chat-container">
      <div className="chat-header">
        <div className="agent-avatar" aria-hidden="true">
          <Bot size={28} />
        </div>
        <div>
          <h2 style={{ margin: 0, fontSize: "1.25rem" }}>OpenClaw Agent</h2>
          <div
            style={{
              display: "flex",
              alignItems: "center",
              gap: "6px",
              fontSize: "0.85rem",
              color: "var(--success)",
            }}
          >
            <CheckCircle2
              size={12}
              fill="var(--success)"
              color="var(--bg-card)"
              aria-hidden="true"
            />{" "}
            Online
          </div>
        </div>
      </div>

      <div
        className="chat-messages"
        role="log"
        aria-label="Chat messages"
        aria-live="polite"
        aria-relevant="additions"
      >
        {messages.map((msg) => (
          <div key={msg.id} className={`message ${msg.sender}`}>
            {msg.proactive && (
              <div
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: "6px",
                  fontSize: "0.75rem",
                  color: "var(--accent-primary)",
                  marginBottom: "4px",
                  fontWeight: 600,
                  textTransform: "uppercase",
                }}
              >
                <AlertCircle size={12} aria-hidden="true" /> Proactive Nudge
              </div>
            )}
            <div className="message-bubble">{msg.text}</div>
          </div>
        ))}

        {isTyping && (
          <div className="message agent" role="status">
            <span className="sr-only">Agent is typing...</span>
            <div className="typing-indicator" aria-hidden="true">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>
        )}

        <div ref={messagesEndRef} />
      </div>

      <form onSubmit={handleSubmit} className="chat-input-container">
        <input
          id="chat-input"
          type="text"
          placeholder={placeholder}
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          aria-label="Message input"
        />
        <button type="submit" className="send-button" aria-label="Send message">
          <Send size={18} aria-hidden="true" />
        </button>
      </form>
    </div>
  );
}
