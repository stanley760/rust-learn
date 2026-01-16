import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { message } from 'antd';
import { ProcessRecord } from '../types/process';

/**
 * 进程管理自定义 Hook
 * 封装进程列表的获取、搜索、终止等操作
 */
export const useProcesses = () => {
  // 状态管理
  const [processes, setProcesses] = useState<ProcessRecord[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  /**
   * 刷新进程列表
   * 调用后端 get_processes 命令获取所有网络连接进程
   */
  const refreshProcesses = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ProcessRecord[]>('get_processes');
      setProcesses(result);
      message.success('已刷新进程列表');
    } catch (err) {
      const errorMsg = `获取进程列表失败: ${err}`;
      setError(errorMsg);
      message.error(errorMsg);
      console.error('Error fetching processes:', err);
    } finally {
      setLoading(false);
    }
  };

  /**
   * 按端口号搜索进程
   * @param port - 端口号字符串
   */
  const searchByPort = async (port: string) => {
    // 验证端口输入
    if (!port.trim()) {
      message.error('端口号不能为空');
      return;
    }

    // 验证端口是否为数字
    if (!/^\d+$/.test(port.trim())) {
      message.error('端口号必须是数字');
      return;
    }

    setLoading(true);
    setError(null);
    try {
      const result = await invoke<ProcessRecord[]>('search_by_port', { port: port.trim() });
      setProcesses(result);
      
      if (result.length === 0) {
        message.info(`未找到使用端口 ${port} 的进程`);
      } else {
        message.success(`找到 ${result.length} 个使用端口 ${port} 的进程`);
      }
    } catch (err) {
      const errorMsg = `搜索失败: ${err}`;
      setError(errorMsg);
      message.error(errorMsg);
    } finally {
      setLoading(false);
    }
  };

  /**
   * 终止指定 PID 的进程
   * @param pid - 进程 ID 字符串
   */
  const killProcess = async (pid: string) => {
    // 验证 PID 输入
    if (!pid.trim()) {
      message.error('PID 不能为空');
      return;
    }

    // 验证 PID 是否为数字
    if (!/^\d+$/.test(pid.trim())) {
      message.error('PID 必须是数字');
      return;
    }

    setLoading(true);
    setError(null);
    try {
      await invoke('kill_process', { pid: pid.trim() });
      message.success(`成功终止进程 ${pid}`);
      
      // 自动刷新进程列表
      await refreshProcesses();
    } catch (err) {
      const errorMsg = `终止进程失败: ${err}`;
      setError(errorMsg);
      message.error(errorMsg);
      setLoading(false);
    }
  };

  // 组件挂载时自动加载进程列表
  useEffect(() => {
    refreshProcesses();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return {
    processes,
    loading,
    error,
    refreshProcesses,
    searchByPort,
    killProcess,
  };
};
