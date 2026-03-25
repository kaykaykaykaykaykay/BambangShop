# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations


After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Do we still need an interface/trait for Subscriber?
In this BambangShop case, a single Model struct is enough. Since there is only one type of Subscriber (the receiver app), there is no need for a trait/interface. A trait would only be necessary if there were multiple different Subscriber types that need to be treated polymorphically.
2. Vec vs DashMap — which is sufficient?
DashMap is necessary. Since id in Product and url in Subscriber must be unique, we need a Map/Dictionary structure to enforce that uniqueness and allow fast O(1) lookups. Using a Vec would require iterating through all elements to check for duplicates or find a specific entry, which is less efficient and doesn't enforce uniqueness naturally.
3. Do we still need DashMap or can we use Singleton pattern instead?
We still need DashMap. The Singleton pattern only ensures a single instance of an object exists — it does not provide thread safety on its own. Since SUBSCRIBERS is accessed by multiple threads simultaneously (due to multi-threaded notifications), we need DashMap's built-in thread-safe concurrent access. Implementing Singleton with a regular HashMap would still require a separate synchronisation mechanism, so DashMap is the better and more practical choice here.
#### Reflection Publisher-2
1. Why separate Service and Repository from Model?
To comply with the Single Responsibility Principle. A Model should only represent the data structure. If we put business logic into Service and data storage operations into Repository, each class has only one reason to change. This makes the code easier to test, maintain, and extend independently.
2. What happens if we only use the Model?
Each model (Program, Subscriber, Notification) would become bloated and tightly coupled to each other. For example, Program would need to know about Subscriber to send notifications, and Subscriber would need to know about Notification to format messages. This increases complexity significantly — any change in one model could break the others, making the code fragile and hard to maintain.
3. How does Postman help?
Postman helps by allowing us to test HTTP endpoints easily without writing a frontend. Features like saved collections, environment variables, and request history make it easy to repeatedly test subscribe, unsubscribe, create product, and publish endpoints. For the Group Project, the automated test scripts and collection sharing features are especially useful for team-based API testing.
#### Reflection Publisher-3
1. Which Observer variation is used?
The Push model is used. The main app (Publisher) actively pushes notification data to each Subscriber by making HTTP POST requests to their URLs whenever an event (create, delete, promote) occurs.
2. Advantages and disadvantages of using Pull model instead?

Advantage: The Publisher would not need to know about each Subscriber's URL or state. Subscribers control when they retrieve data, reducing unnecessary notifications and giving them more control.
Disadvantage: Subscribers would need to continuously poll the Publisher for updates, which is inefficient. Notifications would not be real-time — there would be a delay depending on how frequently Subscribers poll.

3. What happens without multi-threading?
Without multi-threading, the notify() function would send HTTP requests to each Subscriber sequentially (one by one). If one Subscriber is slow or unresponsive, the entire notification process would be blocked, causing the main app to hang and making it unresponsive to other requests until all notifications are sent. Multi-threading ensures each notification is sent concurrently, keeping the app responsive.