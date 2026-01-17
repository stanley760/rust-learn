import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from '../utils/toast';
import { ProcessRecord, PageResponse } from '../types/process';

/**
 * 进程管理自定义 Hook
 * 封装进程列表的获取、搜索、终止等操作，支持无限滚动加载
 */
export const useProcesses = () => {
  // 状态管理
  const [processes, setProcesses] = useState<ProcessRecord[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 分页状态
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize] = useState(20); // 每次加载 20 条
  const [total, setTotal] = useState(0);
  const [hasMore, setHasMore] = useState(true);

  // 搜索状态
  const [searchPort, setSearchPort] = useState<string | null>(null);

  /**
   * 加载更多进程数据
   * @param page - 要加载的页码
   * @param append - 是否追加到现有数据（true）还是替换（false）
   */
  const loadProcesses = useCallback(async (page: number, append: boolean = false) => {
    setLoading(true);
    setError(null);

    try {
      let result: PageResponse<ProcessRecord>;

      if (searchPort) {
        // 搜索模式
        result = await invoke<PageResponse<ProcessRecord>>('search_by_port_paginated', {
          port: searchPort,
          page,
          pageSize,
        });
      } else {
        // 正常模式
        result = await invoke<PageResponse<ProcessRecord>>('get_processes_paginated', {
          page,
          pageSize,
        });
      }

      if (append) {
        // 追加数据
        setProcesses(prev => [...prev, ...result.data]);
      } else {
        // 替换数据
        setProcesses(result.data);
      }

      setCurrentPage(result.page);
      setTotal(result.total);
      setHasMore(result.page < result.total_pages);

    } catch (err) {
      const errorMsg = `获取进程列表失败: ${err}`;
      setError(errorMsg);
      toast.error(errorMsg);
      console.error('Error fetching processes:', err);
    } finally {
      setLoading(false);
    }
  }, [searchPort, pageSize]);

  /**
   * 刷新进程列表（重置到第一页）
   */
  const refreshProcesses = useCallback(async () => {
    setCurrentPage(1);
    setSearchPort(null);
    await loadProcesses(1, false);
    toast.success('已刷新进程列表');
  }, [loadProcesses]);

  /**
   * 加载下一页数据
   */
  const loadMore = useCallback(async () => {
    if (!loading && hasMore) {
      const nextPage = currentPage + 1;
      await loadProcesses(nextPage, true);
    }
  }, [loading, hasMore, currentPage, loadProcesses]);

  /**
   * 按端口号搜索进程
   * @param port - 端口号字符串
   */
  const searchByPort = useCallback(async (port: string) => {
    // 验证端口输入
    if (!port.trim()) {
      toast.error('端口号不能为空');
      return;
    }

    // 验证端口是否为数字
    if (!/^\d+$/.test(port.trim())) {
      toast.error('端口号必须是数字');
      return;
    }

    setSearchPort(port.trim());
    setCurrentPage(1);

    setLoading(true);
    setError(null);

    try {
      const result = await invoke<PageResponse<ProcessRecord>>('search_by_port_paginated', {
        port: port.trim(),
        page: 1,
        pageSize,
      });

      setProcesses(result.data);
      setCurrentPage(result.page);
      setTotal(result.total);
      setHasMore(result.page < result.total_pages);

      if (result.total === 0) {
        toast.info(`未找到使用端口 ${port} 的进程`);
      } else {
        toast.success(`找到 ${result.total} 个使用端口 ${port} 的进程`);
      }
    } catch (err) {
      const errorMsg = `搜索失败: ${err}`;
      setError(errorMsg);
      toast.error(errorMsg);
    } finally {
      setLoading(false);
    }
  }, [pageSize]);

  /**
   * 终止指定 PID 的进程
   * @param pid - 进程 ID 字符串
   */
  const killProcess = useCallback(async (pid: string) => {
    // 验证 PID 输入
    if (!pid.trim()) {
      toast.error('PID 不能为空');
      return;
    }

    // 验证 PID 是否为数字
    if (!/^\d+$/.test(pid.trim())) {
      toast.error('PID 必须是数字');
      return;
    }

    setLoading(true);
    setError(null);

    try {
      await invoke('kill_process', { pid: pid.trim() });
      toast.success(`成功终止进程 ${pid}`);

      // 刷新当前数据
      await refreshProcesses();
    } catch (err) {
      const errorMsg = `终止进程失败: ${err}`;
      setError(errorMsg);
      toast.error(errorMsg);
      setLoading(false);
    }
  }, [refreshProcesses]);

  // 组件挂载时自动加载第一页进程列表
  useEffect(() => {
    loadProcesses(1, false);
  }, [loadProcesses]);

  return {
    processes,
    loading,
    error,
    total,
    hasMore,
    refreshProcesses,
    searchByPort,
    killProcess,
    loadMore,
  };
};
