/**
 * 简单的 Toast 通知工具
 * 使用 Ant Design 的 message 组件
 */
import { message } from 'antd';

export const toast = {
    success: (msg: string) => {
        message.success(msg);
    },
    error: (msg: string) => {
        message.error(msg);
    },
    info: (msg: string) => {
        message.info(msg);
    },
    warning: (msg: string) => {
        message.warning(msg);
    },
};
