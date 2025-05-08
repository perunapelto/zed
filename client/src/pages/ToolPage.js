import React from 'react';
import { useParams, Link } from 'react-router-dom';

const toolNames = {
  'image-gen': 'AI Image Generator',
  'text-summarizer': 'Text Summarizer',
  'code-review': 'Code Review',
  'seo-analyzer': 'SEO Analyzer',
  'chatbot': 'Chatbot Builder',
  'voice-clone': 'Voice Cloner',
  'logo-maker': 'Logo Maker',
  'video-gen': 'Video Generator',
  'sentiment': 'Sentiment Analyzer',
  'translate': 'AI Translator'
};

function ToolPage() {
  const { toolId } = useParams();
  const toolName = toolNames[toolId] || 'Tool';

  return (
    <div style={{ padding: '2rem', textAlign: 'center' }}>
      <h1>{toolName}</h1>
      <p>This is a placeholder for the {toolName} tool.</p>
      <Link to="/">← Back to Home</Link>
    </div>
  );
}

export default ToolPage;
