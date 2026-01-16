import React, { useState } from 'react';
import { Input, Button } from 'antd';
import './ControlPanel.css';

/**
 * ControlPanel 组件属性接口
 */
export interface ControlPanelProps {
  /** 搜索端口回调 */
  onSearch: (port: string) => Promise<void>;
  /** 刷新进程列表回调 */
  onRefresh: () => Promise<void>;
  /** 终止进程回调 */
  onKill: (pid: string) => Promise<void>;
  /** 重置回调 */
  onReset: () => void;
  /** PID 输入框的值（受控组件） */
  pidValue: string;
  /** 设置 PID 输入框的值 */
  setPidValue: (value: string) => void;
}

/**
 * ControlPanel 组件
 * 提供端口搜索、进程终止、刷新和重置功能的控制面板
 * 
 * @param props - 组件属性
 * @returns ControlPanel 组件
 */
export const ControlPanel: React.FC<ControlPanelProps> = ({
  onSearch,
  onRefresh,
  onKill,
  onReset,
  pidValue,
  setPidValue,
}) => {
  // 本地状态：端口输入框的值
  const [portValue, setPortValue] = useState('');

  /**
   * 处理重置操作
   * 清空所有输入框并调用 onReset 回调
   */
  const handleReset = () => {
    setPortValue('');
    setPidValue('');
    onReset();
  };

  return (
    <div className="control-panel">
      <div className="control-row-inline">
        {/* 端口搜索 */}
        <div className="control-item">
          <label className="control-label-inline">PORT:</label>
          <Input
            placeholder="PID:"
            value={portValue}
            onChange={(e) => setPortValue(e.target.value)}
            onPressEnter={() => onSearch(portValue)}
            className="control-input"
            size="small"
            style={{ width: 150 }}
          />
        </div>

        {/* 搜索按钮 */}
        <Button
          onClick={() => onSearch(portValue)}
          type="primary"
          size="small"
        >
          search
        </Button>

        {/* 刷新按钮 */}
        <Button
          onClick={onRefresh}
          type="default"
          size="small"
        >
          refresh
        </Button>

        {/* 重置按钮 */}
        <Button
          onClick={handleReset}
          type="default"
          size="small"
        >
          reset
        </Button>

        {/* PID 输入 */}
        <div className="control-item" style={{ marginLeft: '34px' }}>
          <label className="control-label-inline">PID:</label>
          <Input
            placeholder="输入进程 PID"
            value={pidValue}
            onChange={(e) => setPidValue(e.target.value)}
            onPressEnter={() => onKill(pidValue)}
            className="control-input"
            size="small"
            style={{ width: 150 }}
          />
        </div>

        {/* 终止进程按钮 */}
        <Button
          onClick={() => onKill(pidValue)}
          type="primary"
          danger
          size="small"
        >
          kill
        </Button>
      </div>
    </div>
  );
};
