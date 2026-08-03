# Jira Clone

Project management and issue tracking system built with GritShield, featuring agile board support, workflow automation, and real-time collaboration.

## Overview

This application provides a Jira-like experience for managing software development projects, with support for:

- **Project Management**: Create and manage multiple projects with custom workflows
    
- **Issue Tracking**: Create, assign, and track issues (tasks, bugs, stories)
    
- **Agile Boards**: Kanban boards with sprint support
    
- **Workflow Automation**: Customizable workflow steps with transitions
    
- **User Management**: Role-based access control (Admin, Member, Viewer)
    

## Key Business Features

### Projects

- Create, update, and delete projects
    
- Each project has its own workflow steps
    
- Project-specific boards and backlogs
    
- Member management with role assignments
    

### Issues

- Create issues with types (Bug, Task, Story, Epic)
    
- Assign issues to team members
    
- Track priority (Low, Medium, High)
    
- Add comments to issues
    
- Move issues across workflow steps
    
- Search issues using JQL-like queries
    

### Sprints

- Create sprints with goals and timeframes
    
- Start and manage active sprints
    
- Assign issues to sprints
    
- Sprint burndown tracking
    

### Workflow

- Customizable workflow steps per project
    
- Visual board columns
    
- Step transitions with validation
    
- Workflow management interface
    


# Dependency Injection

Gritshield's **dependency injection** system handles all the dependency wiring — saving dozens of lines of repetitive code.

![A beautiful mountain landscape](/static/di.png)


## Workflow Automation

The project includes an event-driven architecture for business process automation:

### Events

- **IssueCreated**: Triggers when new issues are created
    
- **IssueTransitioned**: Triggers when issues move between workflow steps
    
- **CommentAdded**: Triggers when comments are added to issues
    

### Jobs

- **SendIssueDigestJob**: Background job for email notifications
    
- **GenerateSprintBurndownJob**: Background job for sprint metrics
    

### Event Flow Diagram

![A beautiful mountain landscape](/static/queue.png)

## Service Architecture

The application uses dependency injection for clean separation of concerns:

```text

┌──────────────────────────────────────────────────────────────┐
│                    Controllers Layer                         │
│  ┌────────────┬──────────┬──────────┬──────────┬─────────┐   │
│  │   Auth     │  Board   │  Issue   │ Project  │ Sprint  │   │
│  └────────────┴──────────┴──────────┴──────────┴─────────┘   │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                    Services Layer                            │
│  ┌────────────┬──────────┬──────────┬──────────┬─────────┐   │
│  │ Board      │ Issue    │ Project  │ Workflow │ JQL     │   │
│  └────────────┴──────────┴──────────┴──────────┴─────────┘   │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                    Repositories Layer                        │
│  ┌────────────┬──────────┬──────────┬──────────┬─────────┐   │
│  │ Project    │ Issue    │ Sprint   │ Workflow │ User    │   │
│  └────────────┴──────────┴──────────┴──────────┴─────────┘   │
└──────────────────────────────────────────────────────────────┘
```

## Technology Stack

- **Framework**: Gritshield (custom Rust web framework)
    
- **Database**: SeaORM with PostgreSQL
    
- **Frontend**: HTMX for dynamic updates, Maud for HTML templates
    
- **Events**: Event bus for decoupled communication
    
- **Jobs**: Background job processing
    
- **Security**: Role-based access control, input sanitization
    

## Getting Started

### Installation

1. Clone the repository
    
2. Set up database configuration
    
3. Run database migrations
    
4. Start the application
    
5. Access the web interface at `http://localhost:8080`
    

### Default Login

- Email: admin@example.com
    
- Password: admin123

- Admin Panel: check .env file