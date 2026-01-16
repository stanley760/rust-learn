import React from 'react';
import { Table } from 'antd';
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
}

/**
 * ProcessTable 组件
 * 使用 Ant Design Table 渲染进程列表
 * 
 * @param props - 组件属性
 * @returns ProcessTable 组件
 */
export const ProcessTable: React.FC<ProcessTableProps> = ({
  data,
  loading,
  onRowClick,
}) => {
  // 定义表格列配置
  const columns: ColumnsType<ProcessRecord> = [
    {
      title: '协议',
      dataIndex: 'protocol',
      key: 'protocol',
      width: 80,
      align: 'center',
    },
    {
      title: '本地地址',
      dataIndex: 'local_address',
      key: 'local_address',
      width: 180,
      ellipsis: true,
      align: 'center',
    },
    {
      title: '远程地址',
      dataIndex: 'remote_address',
      key: 'remote_address',
      width: 180,
      ellipsis: true,
      align: 'center',
    },
    {
      title: '状态',
      dataIndex: 'state',
      key: 'state',
      width: 120,
      align: 'center',
    },
    {
      title: 'PID',
      dataIndex: 'pid',
      key: 'pid',
      width: 100,
      align: 'center',
    },
  ];

  return (
    <Table<ProcessRecord>
      columns={columns}
      dataSource={data}
      loading={loading}
      onRow={(record) => ({
        onClick: () => onRowClick(record),
        style: { cursor: 'pointer' },
      })}
      rowKey={(record) => `${record.pid}-${record.local_address}-${record.remote_address}`}
      pagination={{
        pageSize: 10,
        showSizeChanger: true,
        showTotal: (total) => `共 ${total} 条记录`,
        pageSizeOptions: ['10', '20', '50', '100'],
      }}
      size="small"
    />
  );
};
