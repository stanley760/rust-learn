/**
 * 进程记录接口
 * 与后端 Rust ProcessRecord 结构体保持一致
 */
export interface ProcessRecord {
  /** 网络协议（TCP、UDP 等） */
  protocol: string;
  /** 本地地址（IP:Port 格式） */
  local_address: string;
  /** 远程地址（IP:Port 格式） */
  remote_address: string;
  /** 连接状态（LISTENING、ESTABLISHED 等） */
  state: string;
  /** 进程 ID（字符串格式，便于显示） */
  pid: string;
}

/**
 * 分页响应接口
 * 与后端 Rust PageResponse 结构体保持一致
 */
export interface PageResponse<T> {
  /** 当前页数据 */
  data: T[];
  /** 当前页码 */
  page: number;
  /** 每页数量 */
  page_size: number;
  /** 总记录数 */
  total: number;
  /** 总页数 */
  total_pages: number;
}

/**
 * API 错误类型
 * 用于表示后端返回的错误信息
 */
export type ApiError = string;

/**
 * API 响应结果类型
 * 封装成功或失败的响应
 */
export type ApiResult<T> = {
  success: true;
  data: T;
} | {
  success: false;
  error: ApiError;
};
