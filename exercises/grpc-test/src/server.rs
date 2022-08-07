use std::sync::Mutex;
use todo::{
    // 引入从todo.proto编译的Rust struct，可以在/target/debug/build目录中看到它们。
    // 由于Proto中定义了Todo service, 将会自动编译成todo::todo_server下的
    // Todo Trait 和 TodoServer struct
    todo_server::{Todo, TodoServer},
    CreateTodoRequest,
    CreateTodoResponse,
    GetTodosResponse,
    TodoItem,
};
use tonic::transport::Server;

pub mod todo {
    tonic::include_proto!("todo");
}

// 现在我们需要创建TodoService并为该服务实现Todo trait。
#[derive(Debug, Default)]
pub struct TodoService {
    todos: Mutex<Vec<TodoItem>>,
}

#[tonic::async_trait]
impl Todo for TodoService {
    async fn get_todos(
        &self,
        _: tonic::Request<()>,
    ) -> Result<tonic::Response<GetTodosResponse>, tonic::Status> {
        let message = GetTodosResponse {
            todos: self.todos.lock().unwrap().to_vec(),
        };

        Ok(tonic::Response::new(message))
    }

    async fn create_todo(
        &self,
        request: tonic::Request<CreateTodoRequest>,
    ) -> Result<tonic::Response<CreateTodoResponse>, tonic::Status> {
        let payload = request.into_inner();

        let todo_item = TodoItem {
            name: payload.name,
            description: payload.description,
            priority: payload.priority,
            completed: false,
        };

        self.todos.lock().unwrap().push(todo_item.clone());

        let message = CreateTodoResponse {
            todo: Some(todo_item),
            status: true,
        };

        Ok(tonic::Response::new(message))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let todo_service = TodoService::default();

    Server::builder()
        .add_service(TodoServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
