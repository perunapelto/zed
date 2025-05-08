import React from 'react';
import { Link } from 'react-router-dom';
import './Home.css';

const tools = [
  { id: 'image-gen', name: 'AI Image Generator', desc: 'Create images with AI' },
  { id: 'text-summarizer', name: 'Text Summarizer', desc: 'Summarize long texts' },
  { id: 'code-review', name: 'Code Review', desc: 'AI-powered code analysis' },
  { id: 'seo-analyzer', name: 'SEO Analyzer', desc: 'Optimize your website' },
  { id: 'chatbot', name: 'Chatbot Builder', desc: 'Build smart chatbots' },
  { id: 'voice-clone', name: 'Voice Cloner', desc: 'Clone voices with AI' },
  { id: 'logo-maker', name: 'Logo Maker', desc: 'Generate logos instantly' },
  { id: 'video-gen', name: 'Video Generator', desc: 'Create videos from text' },
  { id: 'sentiment', name: 'Sentiment Analyzer', desc: 'Analyze text sentiment' },
  { id: 'translate', name: 'AI Translator', desc: 'Translate between languages' }
];

function Home() {
  return (
    <div className="home">
      <h1>Welcome to ToroGold-Ai Web Services</h1>
      <div className="tools-grid">
        {tools.map(tool => (
          <Link to={`/tool/${tool.id}`} className="tool-card" key={tool.id}>
            <h2>{tool.name}</h2>
            <p>{tool.desc}</p>
          </Link>
        ))}
      </div>
    </div>
  );
}

export default Home;
