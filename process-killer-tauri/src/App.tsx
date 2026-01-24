import { ConfigProvider, message } from 'antd';
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
 * 应用根组件，配置苹果风格的 Liquid Glass 主题
 */
function App() {
  return (
    <ConfigProvider
      theme={{
        algorithm: [],
        token: {
          // 苹果风格配色 - 浅色主题
          colorPrimary: '#007aff',
          colorSuccess: '#34c759',
          colorWarning: '#ff9500',
          colorError: '#ff3b30',
          colorInfo: '#007aff',
          colorBgBase: '#ffffff',

          // 文字颜色
          colorText: '#1d1d1f',
          colorTextSecondary: 'rgba(0, 0, 0, 0.65)',
          colorTextTertiary: 'rgba(0, 0, 0, 0.45)',

          // 圆角和阴影
          borderRadius: 12,
          borderRadiusLG: 16,
          borderRadiusSM: 10,

          // 字体
          fontFamily: '-apple-system, BlinkMacSystemFont, "SF Pro Display", sans-serif',
          fontSize: 14,
          fontSizeHeading1: 32,
          fontSizeHeading2: 28,
          fontSizeHeading3: 24,

          // 间距
          marginXS: 8,
          marginSM: 12,
          margin: 16,
          marginMD: 20,
          marginLG: 24,
          marginXL: 32,
        },
        components: {
          Button: {
            borderRadius: 12,
            fontWeight: 500,
            paddingInline: 20,
            colorPrimary: '#007aff',
            colorDanger: '#ff3b30',
          },
          Input: {
            borderRadius: 12,
            paddingInline: 16,
            colorBgContainer: 'rgba(255, 255, 255, 0.8)',
            colorBorder: 'rgba(0, 0, 0, 0.1)',
            colorText: '#1d1d1f',
            colorTextPlaceholder: 'rgba(0, 0, 0, 0.4)',
          },
          Table: {
            borderRadius: 16,
            colorBgContainer: 'transparent',
            headerColor: '#1d1d1f',
            headerSplitColor: 'rgba(0, 0, 0, 0.08)',
            colorBorderSecondary: 'rgba(0, 0, 0, 0.04)',
          },
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
