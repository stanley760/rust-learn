import React, { useState } from 'react';
import { message } from 'antd';
import { useProcesses } from '../hooks/useProcesses';
import { ProcessTable } from './ProcessTable';
import { ControlPanel } from './ControlPanel';
import { ProcessRecord } from '../types/process';
import './ProcessManager.css';

/**
 * ProcessManager 组件
 * 进程管理主界面，组合控制面板和进程表格
 * 管理组件间状态传递和用户交互事件
 * 
 * @returns ProcessManager 组件
 */
export const ProcessManager: React.FC = () => {
  // 使用 useProcesses Hook 获取进程管理功能
  const {
    processes,
    loading,
    error,
    refreshProcesses,
    searchByPort,
    killProcess,
  } = useProcesses();

  // 本地状态：PID 输入框的值（用于行点击填充）
  const [pidValue, setPidValue] = useState('');

  /**
   * 处理表格行点击事件
   * 将点击行的 PID 填充到 PID 输入框，并清空所有消息
   * 
   * @param record - 被点击的进程记录
   */
  const handleRowClick = (record: ProcessRecord) => {
    // 设置 PID 值
    setPidValue(record.pid);
    
    // 清空所有错误和成功消息
    message.destroy();
  };

  /**
   * 处理刷新操作
   * 刷新进程列表并清空 PID 输入框
   */
  const handleRefresh = async () => {
    // 清空 PID 输入框
    setPidValue('');
    
    // 调用刷新进程列表
    await refreshProcesses();
  };

  /**
   * 处理重置操作
   * 清空 PID 输入框，但不刷新进程列表
   */
  const handleReset = () => {
    // 清空 PID 输入框
    setPidValue('');
    
    // 清空所有消息
    message.destroy();
  };

  return (
    <div className="process-manager">
      <div className="control-panel-container">
        <ControlPanel
          onSearch={searchByPort}
          onRefresh={handleRefresh}
          onKill={killProcess}
          onReset={handleReset}
          pidValue={pidValue}
          setPidValue={setPidValue}
        />
      </div>

      <div className="process-table-container">
        <ProcessTable
          data={processes}
          loading={loading}
          onRowClick={handleRowClick}
        />
      </div>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}
    </div>
  );
};
