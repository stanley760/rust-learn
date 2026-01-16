import { ConfigProvider, message, theme } from 'antd';
import { ProcessManager } from './components/ProcessManager';
import 'antd/dist/reset.css';
import './App.css';

// 配置全局 Message 组件
message.config({
  duration: 3,
  maxCount: 3,
  top: 24,
});

/**
 * App 主组件
 * 应用根组件，配置全局主题和渲染 ProcessManager
 * 使用 Ant Design 暗色主题
 */
function App() {
  return (
    <ConfigProvider
      theme={{
        algorithm: theme.darkAlgorithm,
        token: {
          colorPrimary: '#1890ff',
          borderRadius: 8,
        },
      }}
    >
      <div className="app-container">
        <ProcessManager />
      </div>
    </ConfigProvider>
  );
}

export default App;
