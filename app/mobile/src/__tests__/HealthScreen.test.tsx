import React from 'react';
import { render, waitFor, screen } from '@testing-library/react-native';
import { HealthScreen } from '../screens/HealthScreen';
import { fetchHealthStatus } from '../services/api';
import { config } from '../config';

// Mock the API module
jest.mock('../services/api');
// Mock the config module
jest.mock('../config', () => ({
  config: {
    apiUrl: 'http://localhost:3000',
    envName: 'dev',
    network: 'testnet',
    walletConnectProjectId: 'test-project-id',
    sorobanContractId: 'CC123...',
    isValid: true,
    errors: [],
  },
}));

const mockFetchHealthStatus = fetchHealthStatus as jest.MockedFunction<typeof fetchHealthStatus>;
const mockConfig = config as jest.Mocked<typeof config>;

describe('HealthScreen', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('shows loading state initially', () => {
    mockFetchHealthStatus.mockImplementationOnce(() => new Promise(() => {}));
    
    render(<HealthScreen />);
    
    expect(screen.getByText('Checking system health...')).toBeTruthy();
  });

  it('renders live backend data correctly', async () => {
    const mockData = {
      status: 'ok',
      service: 'backend',
      version: '1.0.0',
      environment: 'development',
      timestamp: new Date().toISOString(),
    };

    mockFetchHealthStatus.mockResolvedValueOnce(mockData);

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('OK')).toBeTruthy();
      expect(screen.getByText('🌐 Live backend data')).toBeTruthy();
      expect(screen.getByText('backend')).toBeTruthy();
      expect(screen.getByText('1.0.0')).toBeTruthy();
    });
  });

  it('shows mock data label when backend fails', async () => {
    mockFetchHealthStatus.mockRejectedValueOnce(new Error('Network error'));

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('🔧 MOCK')).toBeTruthy();
      expect(screen.getByText('📊 Using simulated data')).toBeTruthy();
      expect(screen.getByText('Backend unreachable - showing mock data')).toBeTruthy();
      expect(screen.getByText('⚠️ This is simulated data - backend connection failed')).toBeTruthy();
    });
  });

  it('shows troubleshooting tips when using mock data', async () => {
    mockFetchHealthStatus.mockRejectedValueOnce(new Error('Network error'));

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('🔍 Troubleshooting Tips')).toBeTruthy();
    });
  });

  it('displays the correct mock data structure', async () => {
    mockFetchHealthStatus.mockRejectedValueOnce(new Error('Network error'));

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('backend')).toBeTruthy();
      expect(screen.getByText('0.0.0')).toBeTruthy();
      expect(screen.getByText('development')).toBeTruthy();
      expect(screen.getByText('✅')).toBeTruthy();
      expect(screen.getByText('OK')).toBeTruthy();
    });
  });

  it('shows retry button when error occurs', async () => {
    mockFetchHealthStatus.mockRejectedValueOnce(new Error('Network error'));

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('🔄 Retry Connection')).toBeTruthy();
    });
  });

  // ── Environment indicator tests ─────────────────────────────────────────

  it('shows environment badge in the header', async () => {
    mockFetchHealthStatus.mockResolvedValueOnce({
      status: 'ok', service: 'backend', version: '1.0.0',
      environment: 'development', timestamp: new Date().toISOString(),
    });

    render(<HealthScreen />);

    await waitFor(() => {
      // The env badge element is always rendered
      expect(screen.getByTestId('env-badge')).toBeTruthy();
    });
  });

  it('displays environment label from config', async () => {
    // Note: Since config is mocked as a constant above, we'd need to change the mock 
    // implementation if we wanted to test different values in the same file, 
    // or just verify it shows what's in our default mock.
    mockFetchHealthStatus.mockResolvedValueOnce({
      status: 'ok', service: 'backend', version: '1.0.0',
      environment: 'development', timestamp: new Date().toISOString(),
    });

    render(<HealthScreen />);

    await waitFor(() => {
      // Default mocked envName is 'dev'
      expect(screen.getByText('DEV')).toBeTruthy();
      expect(screen.getByTestId('footer-env-name')).toBeTruthy();
    });
  });

  it('shows blockchain diagnostics section', async () => {
    mockFetchHealthStatus.mockResolvedValueOnce({
      status: 'ok', service: 'backend', version: '1.0.0',
      environment: 'development', timestamp: new Date().toISOString(),
    });

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('Environment & Blockchain')).toBeTruthy();
      expect(screen.getByText('TESTNET')).toBeTruthy();
      expect(screen.getByText('CC123...')).toBeTruthy();
    });
  });

  it('shows configuration errors when config is invalid', async () => {
    // Temporarily modify the mock for this test
    const originalConfig = { ...config };
    (config as any).isValid = false;
    (config as any).errors = ['Missing API Key'];

    mockFetchHealthStatus.mockResolvedValueOnce({
      status: 'ok', service: 'backend', version: '1.0.0',
      environment: 'development', timestamp: new Date().toISOString(),
    });

    render(<HealthScreen />);

    await waitFor(() => {
      expect(screen.getByText('⚠️ Configuration Issues')).toBeTruthy();
      expect(screen.getByText('• Missing API Key')).toBeTruthy();
    });

    // Restore
    Object.assign(config, originalConfig);
  });
});
