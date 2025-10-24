import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './App.css';

interface Account {
  id: string;
  username: string;
  avatar_url?: string;
  auth_method: string;
  created_at: string;
}

interface RepositoryMapping {
  id: string;
  remote_url: string;
  account_id: string;
  remember: boolean;
  created_at: string;
}

interface GitHelperStatus {
  installed: boolean;
  configured: boolean;
}

function App() {
  const [accounts, setAccounts] = useState<Account[]>([]);
  const [mappings, setMappings] = useState<RepositoryMapping[]>([]);
  const [gitHelperStatus, setGitHelperStatus] = useState<GitHelperStatus>({ installed: false, configured: false });
  const [activeTab, setActiveTab] = useState<'accounts' | 'mappings' | 'settings'>('accounts');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Add account form state
  const [newAccountUsername, setNewAccountUsername] = useState('');
  const [newAccountToken, setNewAccountToken] = useState('');

  // Add mapping form state
  const [newMappingUrl, setNewMappingUrl] = useState('');
  const [newMappingAccountId, setNewMappingAccountId] = useState('');
  const [newMappingRemember, setNewMappingRemember] = useState(true);

  useEffect(() => {
    loadData();
  }, []);

  const loadData = async () => {
    setLoading(true);
    try {
      const [accountsData, mappingsData, gitStatus] = await Promise.all([
        invoke<Account[]>('get_accounts'),
        invoke<RepositoryMapping[]>('get_repository_mappings'),
        invoke<GitHelperStatus>('get_git_helper_status')
      ]);
      
      setAccounts(accountsData);
      setMappings(mappingsData);
      setGitHelperStatus(gitStatus);
      setError(null);
    } catch (err) {
      setError(`Failed to load data: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const addAccount = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newAccountUsername || !newAccountToken) {
      setError('Please fill in all fields');
      return;
    }

    setLoading(true);
    try {
      await invoke('add_account', {
        username: newAccountUsername,
        token: newAccountToken
      });
      
      setNewAccountUsername('');
      setNewAccountToken('');
      setSuccess('Account added successfully!');
      await loadData();
    } catch (err) {
      setError(`Failed to add account: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const removeAccount = async (accountId: string) => {
    if (!confirm('Are you sure you want to remove this account?')) return;

    setLoading(true);
    try {
      await invoke('remove_account', { accountId });
      setSuccess('Account removed successfully!');
      await loadData();
    } catch (err) {
      setError(`Failed to remove account: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const testConnection = async (username: string) => {
    setLoading(true);
    try {
      const result = await invoke<{ success: boolean; message: string; scopes?: string[] }>('test_connection', { username });
      if (result.success) {
        setSuccess(result.message);
      } else {
        setError(result.message);
      }
    } catch (err) {
      setError(`Connection test failed: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const addMapping = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newMappingUrl || !newMappingAccountId) {
      setError('Please fill in all fields');
      return;
    }

    setLoading(true);
    try {
      await invoke('set_repository_mapping', {
        remoteUrl: newMappingUrl,
        accountId: newMappingAccountId,
        remember: newMappingRemember
      });
      
      setNewMappingUrl('');
      setNewMappingAccountId('');
      setNewMappingRemember(true);
      setSuccess('Repository mapping added successfully!');
      await loadData();
    } catch (err) {
      setError(`Failed to add mapping: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const removeMapping = async (mappingId: string) => {
    if (!confirm('Are you sure you want to remove this mapping?')) return;

    setLoading(true);
    try {
      await invoke('remove_repository_mapping', { mappingId });
      setSuccess('Mapping removed successfully!');
      await loadData();
    } catch (err) {
      setError(`Failed to remove mapping: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const installGitHelper = async () => {
    setLoading(true);
    try {
      await invoke('install_git_helper');
      setSuccess('Git credential helper installed successfully!');
      await loadData();
    } catch (err) {
      setError(`Failed to install Git helper: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const clearMessages = () => {
    setError(null);
    setSuccess(null);
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>GitSwitchHub</h1>
        <p>Seamless GitHub account switching for macOS</p>
      </header>

      <nav className="tab-nav">
        <button 
          className={activeTab === 'accounts' ? 'active' : ''}
          onClick={() => setActiveTab('accounts')}
        >
          Accounts
        </button>
        <button 
          className={activeTab === 'mappings' ? 'active' : ''}
          onClick={() => setActiveTab('mappings')}
        >
          Repository Mappings
        </button>
        <button 
          className={activeTab === 'settings' ? 'active' : ''}
          onClick={() => setActiveTab('settings')}
        >
          Settings
        </button>
      </nav>

      {error && (
        <div className="alert error">
          <span>{error}</span>
          <button onClick={clearMessages}>×</button>
        </div>
      )}

      {success && (
        <div className="alert success">
          <span>{success}</span>
          <button onClick={clearMessages}>×</button>
        </div>
      )}

      <main className="app-content">
        {activeTab === 'accounts' && (
          <div className="tab-content">
            <div className="section-header">
              <h2>GitHub Accounts</h2>
              <button onClick={loadData} disabled={loading}>
                {loading ? 'Loading...' : 'Refresh'}
              </button>
            </div>

            <div className="accounts-list">
              {accounts.map(account => (
                <div key={account.id} className="account-card">
                  <div className="account-info">
                    {account.avatar_url && (
                      <img src={account.avatar_url} alt={account.username} className="avatar" />
                    )}
                    <div>
                      <h3>{account.username}</h3>
                      <p>Added: {new Date(account.created_at).toLocaleDateString()}</p>
                    </div>
                  </div>
                  <div className="account-actions">
                    <button 
                      onClick={() => testConnection(account.username)}
                      disabled={loading}
                    >
                      Test Connection
                    </button>
                    <button 
                      onClick={() => removeAccount(account.id)}
                      disabled={loading}
                      className="danger"
                    >
                      Remove
                    </button>
                  </div>
                </div>
              ))}
            </div>

            <form onSubmit={addAccount} className="add-form">
              <h3>Add New Account</h3>
              <div className="form-group">
                <label htmlFor="username">GitHub Username:</label>
                <input
                  id="username"
                  type="text"
                  value={newAccountUsername}
                  onChange={(e) => setNewAccountUsername(e.target.value)}
                  placeholder="Enter GitHub username"
                  required
                />
              </div>
              <div className="form-group">
                <label htmlFor="token">Personal Access Token:</label>
                <input
                  id="token"
                  type="password"
                  value={newAccountToken}
                  onChange={(e) => setNewAccountToken(e.target.value)}
                  placeholder="Enter GitHub personal access token"
                  required
                />
                <small>
                  Create a token at <a href="https://github.com/settings/tokens" target="_blank" rel="noopener noreferrer">GitHub Settings</a>
                </small>
              </div>
              <button type="submit" disabled={loading}>
                {loading ? 'Adding...' : 'Add Account'}
              </button>
            </form>
          </div>
        )}

        {activeTab === 'mappings' && (
          <div className="tab-content">
            <div className="section-header">
              <h2>Repository Mappings</h2>
              <button onClick={loadData} disabled={loading}>
                {loading ? 'Loading...' : 'Refresh'}
              </button>
            </div>

            <div className="mappings-list">
              {mappings.map(mapping => {
                const account = accounts.find(a => a.id === mapping.account_id);
                return (
                  <div key={mapping.id} className="mapping-card">
                    <div className="mapping-info">
                      <h3>{mapping.remote_url}</h3>
                      <p>Account: {account?.username || 'Unknown'}</p>
                      <p>Remember: {mapping.remember ? 'Yes' : 'No'}</p>
                    </div>
                    <button 
                      onClick={() => removeMapping(mapping.id)}
                      disabled={loading}
                      className="danger"
                    >
                      Remove
                    </button>
                  </div>
                );
              })}
            </div>

            <form onSubmit={addMapping} className="add-form">
              <h3>Add Repository Mapping</h3>
              <div className="form-group">
                <label htmlFor="mapping-url">Repository URL:</label>
                <input
                  id="mapping-url"
                  type="text"
                  value={newMappingUrl}
                  onChange={(e) => setNewMappingUrl(e.target.value)}
                  placeholder="https://github.com/owner/repo"
                  required
                />
              </div>
              <div className="form-group">
                <label htmlFor="mapping-account">Account:</label>
                <select
                  id="mapping-account"
                  value={newMappingAccountId}
                  onChange={(e) => setNewMappingAccountId(e.target.value)}
                  required
                >
                  <option value="">Select an account</option>
                  {accounts.map(account => (
                    <option key={account.id} value={account.id}>
                      {account.username}
                    </option>
                  ))}
                </select>
              </div>
              <div className="form-group">
                <label>
                  <input
                    type="checkbox"
                    checked={newMappingRemember}
                    onChange={(e) => setNewMappingRemember(e.target.checked)}
                  />
                  Remember this mapping
                </label>
              </div>
              <button type="submit" disabled={loading}>
                {loading ? 'Adding...' : 'Add Mapping'}
              </button>
            </form>
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="tab-content">
            <div className="section-header">
              <h2>Settings</h2>
            </div>

            <div className="settings-section">
              <h3>Git Credential Helper</h3>
              <div className="setting-item">
                <div className="setting-info">
                  <h4>Status</h4>
                  <p>
                    {gitHelperStatus.configured ? 'Configured' : 'Not configured'}
                  </p>
                </div>
                <button 
                  onClick={installGitHelper}
                  disabled={loading || gitHelperStatus.configured}
                >
                  {loading ? 'Installing...' : 'Install Helper'}
                </button>
              </div>
            </div>

            <div className="settings-section">
              <h3>About</h3>
              <p>
                GitSwitchHub allows you to seamlessly switch between multiple GitHub accounts 
                when working with different repositories. It automatically manages your Git 
                credentials and repository mappings.
              </p>
            </div>
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
