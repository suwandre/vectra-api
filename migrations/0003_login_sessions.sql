CREATE TABLE login_sessions (
  wallet_address TEXT PRIMARY KEY,
  nonce TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT now()
);