use kovi::bot::runtimebot::{
    CanSendApi, send_api_request_with_forget, send_api_request_with_response,
};
use kovi::bot::{ApiReturn, SendApi};
use serde_json::json;

/// File APIs
pub trait MilkyFileApi: CanSendApi {
    /// 上传私聊文件
    fn upload_private_file(
        &self,
        user_id: i64,
        file_uri: &str,
        file_name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "upload_private_file",
            json!({"user_id": user_id, "file_uri": file_uri, "file_name": file_name}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 上传群文件
    fn upload_group_file(
        &self,
        group_id: i64,
        parent_folder_id: &str,
        file_uri: &str,
        file_name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "upload_group_file",
            json!({"group_id": group_id, "parent_folder_id": parent_folder_id, "file_uri": file_uri, "file_name": file_name}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 获取私聊文件下载链接
    ///
    /// 默认按非本人发送处理（`is_self_send = false`）。
    /// 若文件由本人发送，请使用 [`Self::get_private_file_download_url_with_self_send`]。
    fn get_private_file_download_url(
        &self,
        user_id: i64,
        file_id: &str,
        file_hash: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        self.get_private_file_download_url_with_self_send(user_id, file_id, file_hash, false)
    }

    /// 获取私聊文件下载链接
    ///
    /// `is_self_send` 表示文件是否由本人发送。
    fn get_private_file_download_url_with_self_send(
        &self,
        user_id: i64,
        file_id: &str,
        file_hash: &str,
        is_self_send: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "get_private_file_download_url",
            json!({"user_id": user_id, "file_id": file_id, "file_hash": file_hash, "is_self_send": is_self_send}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 获取群文件下载链接
    fn get_group_file_download_url(
        &self,
        group_id: i64,
        file_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "get_group_file_download_url",
            json!({"group_id": group_id, "file_id": file_id}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 获取群文件列表
    fn get_group_files(
        &self,
        group_id: i64,
        parent_folder_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "get_group_files",
            json!({"group_id": group_id, "parent_folder_id": parent_folder_id}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 移动群文件
    fn move_group_file(
        &self,
        group_id: i64,
        file_id: &str,
        parent_folder_id: &str,
        target_folder_id: &str,
    ) {
        let send_api = SendApi::new(
            "move_group_file",
            json!({"group_id": group_id, "file_id": file_id, "parent_folder_id": parent_folder_id, "target_folder_id": target_folder_id}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }

    /// 重命名群文件
    fn rename_group_file(
        &self,
        group_id: i64,
        file_id: &str,
        parent_folder_id: &str,
        new_file_name: &str,
    ) {
        let send_api = SendApi::new(
            "rename_group_file",
            json!({"group_id": group_id, "file_id": file_id, "parent_folder_id": parent_folder_id, "new_file_name": new_file_name}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }

    /// 删除群文件
    fn delete_group_file(&self, group_id: i64, file_id: &str) {
        let send_api = SendApi::new(
            "delete_group_file",
            json!({"group_id": group_id, "file_id": file_id}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }

    /// 转存群文件为永久文件
    fn persist_group_file(&self, group_id: i64, file_id: &str) {
        let send_api = SendApi::new(
            "persist_group_file",
            json!({"group_id": group_id, "file_id": file_id}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }

    /// 创建群文件夹
    fn create_group_folder(
        &self,
        group_id: i64,
        folder_name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> {
        let send_api = SendApi::new(
            "create_group_folder",
            json!({"group_id": group_id, "folder_name": folder_name}),
        );
        send_api_request_with_response(self.__get_api_tx(), send_api)
    }

    /// 重命名群文件夹
    fn rename_group_folder(&self, group_id: i64, folder_id: &str, new_folder_name: &str) {
        let send_api = SendApi::new(
            "rename_group_folder",
            json!({"group_id": group_id, "folder_id": folder_id, "new_folder_name": new_folder_name}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }

    /// 删除群文件夹
    fn delete_group_folder(&self, group_id: i64, folder_id: &str) {
        let send_api = SendApi::new(
            "delete_group_folder",
            json!({"group_id": group_id, "folder_id": folder_id}),
        );
        send_api_request_with_forget(self.__get_api_tx(), send_api);
    }
}
