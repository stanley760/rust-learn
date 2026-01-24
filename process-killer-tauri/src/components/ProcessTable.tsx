import React, { useRef, useEffect, useCallback } from 'react';
import { Table, Spin } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import { ProcessRecord } from '../types/process';

/**
 * ProcessTable 组件属性接口
 */
export interface ProcessTableProps {
  /** 进程数据数组 */
  data: ProcessRecord[];
  /** 加载状态 */
  loading: boolean;
  /** 行点击事件回调 */
  onRowClick: (record: ProcessRecord) => void;
  /** 总记录数 */
  total: number;
  /** 是否还有更多数据 */
  hasMore: boolean;
  /** 加载更多回调 */
  onLoadMore: () => void;
}

/**
 * ProcessTable 组件
 * 使用 Ant Design Table 渲染进程列表，支持无限滚动加载
 * 
 * @param props - 组件属性
 * @returns ProcessTable 组件
 */
export const ProcessTable: React.FC<ProcessTableProps> = ({
  data,
  loading,
  onRowClick,
  total,
  hasMore,
  onLoadMore,
}) => {
  const tableRef = useRef<HTMLDivElement>(null);
  const loadingRef = useRef(false);
  const scrollContainerRef = useRef<HTMLElement | null>(null);

  // 定义表格列配置
  const columns: ColumnsType<ProcessRecord> = [
    {
      title: '协议',
      dataIndex: 'protocol',
      key: 'protocol',
      width: 70,
      align: 'center',
    },
    {
      title: '本地地址',
      dataIndex: 'local_address',
      key: 'local_address',
      width: 180,
      ellipsis: { showTitle: false },
      render: (text: string) => (
        <div style={{ fontSize: '12px', fontFamily: 'monospace' }} title={text}>
          {text}
        </div>
      ),
    },
    {
      title: '远程地址',
      dataIndex: 'remote_address',
      key: 'remote_address',
      width: 180,
      ellipsis: { showTitle: false },
      render: (text: string) => (
        <div style={{ fontSize: '12px', fontFamily: 'monospace' }} title={text}>
          {text}
        </div>
      ),
    },
    {
      title: '状态',
      dataIndex: 'state',
      key: 'state',
      width: 100,
      ellipsis: true,
      align: 'center',
    },
    {
      title: 'PID',
      dataIndex: 'pid',
      key: 'pid',
      width: 80,
      align: 'center',
    },
  ];

  // 滚动处理函数
  const handleScroll = useCallback((e: Event) => {
    const target = e.target as HTMLElement;
    if (!target) return;

    const scrollTop = target.scrollTop;
    const scrollHeight = target.scrollHeight;
    const clientHeight = target.clientHeight;

    const distanceToBottom = scrollHeight - scrollTop - clientHeight;

    // 调试信息
    if (data.length > 0 && data.length % 20 === 0) {
      console.log('Scroll Info:', {
        scrollTop,
        scrollHeight,
        clientHeight,
        distanceToBottom,
        hasMore,
        loading,
        loadingRefCurrent: loadingRef.current,
        dataLength: data.length,
      });
    }

    // 当滚动到距离底部 200px 时触发加载
    if (
      distanceToBottom < 200 &&
      hasMore &&
      !loading &&
      !loadingRef.current
    ) {
      console.log('🚀 Triggering load more...');
      loadingRef.current = true;
      onLoadMore();

      // 500ms 后重置防抖标志
      setTimeout(() => {
        loadingRef.current = false;
      }, 500);
    }
  }, [hasMore, loading, onLoadMore, data.length]);

  // 设置滚动监听器
  useEffect(() => {
    // 延迟查找滚动容器，确保 DOM 已渲染
    const timer = setTimeout(() => {
      const tableElement = tableRef.current;
      if (!tableElement) {
        console.warn('❌ Table ref not found');
        return;
      }

      const tableBody = tableElement.querySelector('.ant-table-body') as HTMLElement;
      if (!tableBody) {
        console.warn('❌ .ant-table-body not found');
        return;
      }

      console.log('✅ Scroll listener attached to .ant-table-body');
      console.log('Initial scroll info:', {
        scrollHeight: tableBody.scrollHeight,
        clientHeight: tableBody.clientHeight,
        hasScroll: tableBody.scrollHeight > tableBody.clientHeight,
      });

      scrollContainerRef.current = tableBody;
      tableBody.addEventListener('scroll', handleScroll);
    }, 100);

    return () => {
      clearTimeout(timer);
      if (scrollContainerRef.current) {
        scrollContainerRef.current.removeEventListener('scroll', handleScroll);
        console.log('🔌 Scroll listener removed');
      }
    };
  }, [handleScroll]);

  // 当数据变化时，检查是否需要自动加载更多（如果内容不足以产生滚动条）
  useEffect(() => {
    const checkAndLoadMore = () => {
      const tableBody = scrollContainerRef.current;
      if (!tableBody || loading || !hasMore || loadingRef.current) return;

      const hasScroll = tableBody.scrollHeight > tableBody.clientHeight;

      if (!hasScroll && data.length > 0) {
        console.log('📏 Content too short, auto-loading more...');
        loadingRef.current = true;
        onLoadMore();
        setTimeout(() => {
          loadingRef.current = false;
        }, 500);
      }
    };

    // 延迟检查，确保 DOM 已更新
    const timer = setTimeout(checkAndLoadMore, 200);
    return () => clearTimeout(timer);
  }, [data.length, loading, hasMore, onLoadMore]);

  return (
    <div ref={tableRef} style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
      <Table<ProcessRecord>
        columns={columns}
        dataSource={data}
        loading={loading && data.length === 0}
        onRow={(record) => ({
          onClick: () => onRowClick(record),
          style: { cursor: 'pointer' },
        })}
        rowKey={(record) => `${record.pid}-${record.local_address}-${record.remote_address}`}
        pagination={false}
        scroll={{ y: 'calc(100vh - 200px)' }}
        size="small"
        footer={() => (
          <div style={{
            textAlign: 'center',
            padding: '8px 16px',
            marginBottom: '-4px',
            background: '#ffffff',
            border: 'none',
            borderTop: '1px solid rgba(0, 0, 0, 0.06)',
            boxShadow: 'none',
            color: 'rgba(0, 0, 0, 0.65)',
            fontSize: '13px',
            borderRadius: '0 0 16px 16px'
          }}>
            {loading && data.length > 0 && (
              <Spin size="small" style={{ marginRight: 8 }} />
            )}
            <span>
              已加载 {data.length} / {total} 条记录
              {hasMore && !loading && ' - 向下滚动加载更多'}
              {!hasMore && data.length > 0 && ' - 已加载全部数据'}
            </span>
          </div>
        )}
      />
    </div>
  );
};
