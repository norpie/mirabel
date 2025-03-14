import type { PlanItem, Session, ShallowSession } from "$lib/models/session";
import type { Page, PageResponse } from "$lib/models/page";
import { generateId } from "$lib/utils";

/*
# User History Page Specification (Final)

## Overview
This feature adds a "History" page where users can view their past actions.

## Scope
- Users should be able to see their own history.
- The system will track actions that modify data: `CREATE`, `UPDATE`, and `DELETE`.

## Data Storage
- **New database table:** `user_action_history`
  - `id`: Primary key
  - `user_id`: Foreign key to `users` table
  - `timestamp`: Timestamp of the action
  - `action_type`: Enum (`CREATE`, `UPDATE`, `DELETE`)
  - `entity`: Affected entity (e.g., "Project", "Task")
  - `description`: Short summary of the action

## UI
- **Route:** `/history` (Only displays the logged-in user's actions)
- **Pagination:** 20 items per page
- **Columns:** Timestamp, Action Type, Entity, Description
- **Filters:** Action Type dropdown, Date Range picker
- **Empty State:** "No recent activity yet."
- **Export Option:** "Download CSV" button

## API
- `GET /api/history`
  - **Query parameters:**
    - `page`: Page number (default: 1)
    - `limit`: Number of items per page (default: 20)
    - `action_type`: Optional filter (`CREATE`, `UPDATE`, `DELETE`)
    - `start_date`, `end_date`: Optional date range filter
- `GET /api/history/export`
  - **Format:** CSV
  - **Mirabelncludes applied filters:** Yes

## Permissions
- Read-only for now. Users cannot delete history.
- Only logged-in users can access the history page.
*/
const spec = `
# User History Page Specification (Final)

## Overview
This feature adds a "History" page where users can view their past actions.

## Scope
- Users should be able to see their own history.
- The system will track actions that modify data: \`CREATE\`, \`UPDATE\`, and \`DELETE\`.

## Data Storage
- **New database table:** \`user_action_history\`
  - \`id\`: Primary key
  - \`user_id\`: Foreign key to \`users\` table
  - \`timestamp\`: Timestamp of the action
  - \`action_type\`: Enum (\`CREATE\`, \`UPDATE\`, \`DELETE\`)
  - \`entity\`: Affected entity (e.g., "Project", "Task")
  - \`description\`: Short summary of the action

## UI
- **Route:** \`/history\` (Only displays the logged-in user's actions)
- **Pagination:** 20 items per page
- **Columns:** Timestamp, Action Type, Entity, Description
- **Filters:** Action Type dropdown, Date Range picker
- **Empty State:** "No recent activity yet."
- **Export Option:** "Download CSV" button

## API
- \`GET /api/history\`
  - **Query parameters:**
    - \`page\`: Page number (default: 1)
    - \`limit\`: Number of items per page (default: 20)
    - \`action_type\`: Optional filter (\`CREATE\`, \`UPDATE\`, \`DELETE\`)
    - \`start_date\`, \`end_date\`: Optional date range filter
- \`GET /api/history/export\`
  - **Format:** CSV
  - **Includes applied filters:** Yes

## Permissions
- Read-only for now. Users cannot delete history.
- Only logged-in users can access the history page.`;

/*
1. Database Schema Setup
    1.1. Create Migration
        1.1.1. Run `migrate create -n create_user_action_history`
        1.1.2. Patch `migrations/*_create_user_action_history.sql`:
            1.1.2.1. CREATE TYPE action_type AS ENUM ('CREATE', 'UPDATE', 'DELETE')
            1.1.2.2. CREATE TABLE user_action_history (...)
    1.2. Apply Migration
        1.2.1. Run `db-migrate up`
    1.3. Create Indexes
        1.3.1. Run `psql -c "CREATE INDEX idx_user_actions ON user_action_history (user_id, timestamp)"`
2. Backend API Implementation
    2.1. History Route
        2.1.1. Run `nest generate controller history`
        2.1.2. Patch `src/history/history.controller.ts`:
            2.1.2.1. Add @Get() decorator
            2.1.2.2. Implement query params validation
    2.2. Query Service
        2.2.1. Patch `src/history/history.service.ts`:
            2.2.1.1. Implement getUserHistory() with pagination
            2.2.1.2. Add date filtering logic
    2.3. CSV Export
        2.3.1. Run `npm install csv-stringify`
        2.3.2. Patch `src/history/history.controller.ts`:
            2.3.2.1. Add @Get('export') endpoint
            2.3.2.2. Implement CSV serialization
3. Frontend Implementation
    3.1. History Page Component
        3.1.1. Run `ng generate component history-page`
        3.1.2. Patch `src/app/history-page/history-page.component.html`:
            3.1.2.1. Add filter controls
            3.1.2.2. Implement data table markup
    3.2. API Integration
        3.2.1. Patch `src/app/history.service.ts`:
            3.2.1.1. Add getHistory() method
            3.2.1.2. Implement pagination params
    3.3. Export Button
        3.3.1. Patch `src/app/history-page/history-page.component.ts`:
            3.3.1.1. Add exportToCSV() method
            3.3.1.2. Trigger blob download
4. Permissions & Security
    4.1. Route Guard
        4.1.1. Run `ng generate guard auth`
        4.1.2. Patch `src/app/auth.guard.ts`:
            4.1.2.1. Implement canActivate check
    4.2. API Validation
        4.2.1. Patch `src/history/history.controller.ts`:
            4.2.1.1. Add @UseGuards(JwtAuthGuard)
            4.2.1.2. Verify user ID matches session
5. Testing & Validation
    5.1. Database Tests
        5.1.1. Run `psql -c "INSERT INTO user_action_history (...) VALUES (...)"`
        5.1.2. Run `psql -c "SELECT * FROM user_action_history WHERE ..."`
    5.2. API Tests
        5.2.1. Run `curl -X GET "http://localhost:3000/api/history?page=2"`
        5.2.2. Run `curl -X GET "http://localhost:3000/api/history/export"`
    5.3. UI Tests
        5.3.1. Run `cypress run --spec "history-page.spec.js"`
    5.4. Security Tests
        5.4.1. Run `npx snyk test`
        5.4.2. Run `curl -X GET "http://localhost:3000/api/history" -H "Authorization: Bearer invalid"`
*/
const children: PlanItem[] = [
    {
        id: generateId(),
        name: "Database Schema Setup",
        description: "Define and migrate the database schema for user action history.",
        status: "done",
        children: [
            {
                id: generateId(),
                name: "Create Migration",
                description: "Generate and modify migration scripts.",
                status: "done",
                children: [
                    {
                        id: generateId(),
                        name: "Run migration",
                        description: "Run `migrate create -n create_user_action_history`",
                        status: "done",
                        children: []
                    },
                    {
                        id: generateId(),
                        name: "Patch migration file",
                        description: "Modify the migration file to include necessary tables and enums.",
                        status: "done",
                        children: [
                            {
                                id: generateId(),
                                name: "Define enum",
                                description: "CREATE TYPE action_type AS ENUM ('CREATE', 'UPDATE', 'DELETE')",
                                status: "done",
                                children: []
                            },
                            {
                                id: generateId(),
                                name: "Create table",
                                description: "CREATE TABLE user_action_history (...)",
                                status: "done",
                                children: []
                            }
                        ]
                    }
                ]
            },
            {
                id: generateId(),
                name: "Apply Migration",
                description: "Execute the migration to update the database schema.",
                status: "done",
                children: [
                    {
                        id: generateId(),
                        name: "Run `db-migrate up`",
                        description: "Apply pending database migrations.",
                        status: "done",
                        children: []
                    }
                ]
            },
            {
                id: generateId(),
                name: "Create Indexes",
                description: "Optimize database queries with indexing.",
                status: "done",
                children: [
                    {
                        id: generateId(),
                        description: "Run `psql -c \"CREATE INDEX idx_user_actions ON user_action_history (user_id, timestamp)\"`",
                        name: "Create index",
                        status: "done",
                        children: []
                    }
                ]
            }
        ]
    },
    {
        id: generateId(),
        name: "Backend API Implementation",
        description: "Develop API endpoints for retrieving and exporting user action history.",
        status: "in-progress",
        children: [
            {
                id: generateId(),
                name: "History Route",
                description: "Create the API route for fetching user history.",
                status: "in-progress",
                children: [
                    {
                        id: generateId(),
                        name: "Generate controller",
                        description: "Generate a NestJS controller for handling history requests by running `nest generate controller history`",
                        status: "done",
                        children: []
                    },
                    {
                        id: generateId(),
                        name: "Modify history controller",
                        description: "Modify `src/history/history.controller.ts` to add necessary endpoints and validation.",
                        status: "in-progress",
                        children: [
                            {
                                id: generateId(),
                                name: "Add @Get() decorator",
                                description: "Define a GET endpoint for retrieving history.",
                                status: "done",
                                children: []
                            },
                            {
                                id: generateId(),
                                name: "Implement query params validation",
                                description: "Ensure request parameters are validated before processing.",
                                status: "todo",
                                children: []
                            }
                        ]
                    }
                ]
            }
        ]
    },
    {
        id: generateId(),
        name: "Frontend Implementation",
        description: "Build the UI for displaying and exporting user action history.",
        status: "todo",
        children: [
            {
                id: generateId(),
                name: "History Page Component",
                description: "Create and style the history page UI component.",
                status: "todo",
                children: []
            }
        ]
    },
    {
        id: generateId(),
        name: "Permissions & Security",
        description: "Ensure only authorized users can access the history API.",
        status: "todo",
        children: [
            {
                id: generateId(),
                name: "Activate middleware",
                description: "Add authentication middleware to protect the history route.",
                status: "todo",
                children: []
            }
        ]
    },
    {
        id: generateId(),
        name: "Testing & Validation",
        description: "Perform tests on the database, API, UI, and security.",
        status: "todo",
        children: [
            {
                id: generateId(),
                name: "Write Unit Tests",
                description: "Develop unit tests for API endpoints and services.",
                status: "todo",
                children: []
            }
        ]
    }
];

const sampleResponse: PageResponse<Session[]> = {
    pageInfo: {
        page: 1,
        pageSize: 10,
    },
    data: [
        {
            id: 'sdafopihv',
            title: 'Setting up the workspace',
            participants: [
                {
                    id: 'sadlkjrfhycnsf',
                    name: 'norpie',
                    user: true,
                    avatar: 'https://avatars.githubusercontent.com/u/46564751?v=4'
                },
                {
                    id: 'vasudfniunyave',
                    name: 'Mirabel',
                    user: false
                }
            ],
            plan: {
                id: generateId(),
                goal: 'Implement the user history page',
                spec,
                children
            },
            chat: {
                messages: [
                    {
                        timestamp: '2021-08-31T12:00:00Z',
                        participant: "sadlkjrfhycnsf",
                        message: "Let\'s fix issue #5",
                    },
                    {
                        timestamp: '2021-09-31T12:01:00Z',
                        participant: "vasudfniunyave",
                        message: "I'm on it.",
                    },
                    {
                        timestamp: '2021-09-01T12:02:00Z',
                        participant: 'vasudfniunyave',
                        message: "I've generated a plan. Do you want to approve it?",
                    },
                    {
                        timestamp: '2021-09-01T12:03:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Yes, please.",
                    },
                    {
                        timestamp: '2021-09-01T12:04:00Z',
                        participant: 'vasudfniunyave',
                        message: "Great, I'll start working on it right away."
                    },
                    {
                        timestamp: '2021-09-01T12:05:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Thanks. Let me know if you need any help."
                    },
                    {
                        timestamp: '2021-09-01T12:06:00Z',
                        participant: 'vasudfniunyave',
                        message: "Will do! I'll keep you updated on my progress."
                    },
                    {
                        timestamp: '2021-09-01T12:07:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Sounds good. Take your time."
                    },
                    {
                        timestamp: '2021-09-01T12:08:00Z',
                        participant: 'vasudfniunyave',
                        message: "I've made some progress and am running into a couple of issues."
                    },
                    {
                        timestamp: '2021-09-01T12:09:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Can you tell me more about the issues?"
                    },
                    {
                        timestamp: '2021-09-01T12:10:00Z',
                        participant: 'vasudfniunyave',
                        message: "Sure, I'll explain the issues in detail."
                    },
                    {
                        timestamp: '2021-09-01T12:11:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Great, I'm ready to help."
                    },
                    {
                        timestamp: '2021-09-01T12:12:00Z',
                        participant: 'vasudfniunyave',
                        message: "First, the API call is not returning the expected data."
                    },
                    {
                        timestamp: '2021-09-01T12:13:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Can you check the API endpoint and parameters?"
                    },
                    {
                        timestamp: '2021-09-01T12:14:00Z',
                        participant: 'vasudfniunyave',
                        message: "I did, everything looks correct. Could it be a backend issue?"
                    },
                    {
                        timestamp: '2021-09-01T12:15:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Let's check with the backend team."
                    },
                    {
                        timestamp: '2021-09-01T12:16:00Z',
                        participant: 'vasudfniunyave',
                        message: "Okay, I'll wait for their response."
                    },
                    {
                        timestamp: '2021-09-01T12:17:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Great, thanks."
                    },
                    {
                        timestamp: '2025-02-01T12:18:00Z',
                        participant: 'vasudfniunyave',
                        message: "Also, the UI is not rendering the chat messages as expected."
                    },
                    {
                        timestamp: '2025-02-11T12:19:00Z',
                        participant: 'sadlkjrfhycnsf',
                        message: "Check the component and ensure data binding is correct."
                    }
                ],
            }
        }
    ]
};

export async function fetchSession(sessionId: string): Promise<Session> {
    setTimeout(() => { }, 1000);
    return sampleResponse.data[0];
}

export async function fetchAllSessions(workspaceId: string, page: Page): Promise<PageResponse<ShallowSession[]>> {
    setTimeout(() => { }, 1000);
    return sampleResponse;
}
