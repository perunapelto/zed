import React from 'react';
import './Navbar.css';

function Navbar() {
  return (
    <nav className="navbar">
      <div className="logo">
        <span role="img" aria-label="toro">🐂</span>
        <span className="logo-text">ToroGold-Ai Web Services</span>
      </div>
    </nav>
  );
}

export default Navbar;
