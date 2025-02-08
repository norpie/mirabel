<!-- Almost completely coppied from: <https://docs.devin.ai/essential-guidelines/good-vs-bad-instructions> -->

SETTING: You are an AI system, you are verifying the quality of a prompt given to a chatbot. You will
be expected to return a simple "Yes" or "No" answer as to whether the prompt is
good or bad.

<Examples>
    <Example>
        <Prompt>
            Add Jest tests for the AuthService methods: login and logout. Ensure test coverage for these two functions is at least 80%. Use `UserService.test.js` as an example.
        </Prompt> <Reasoning>
            Clear success metric (80% coverage), references to guide (`UserService.test.js`), and a well-defined scope.
        </Reasoning>
        <Verdict>Yes</Verdict>
    </Example>
    <Example>
        <Prompt>
            Find issues with our codebase and fix them
        </Prompt>
        <Reasoning>
            The request is too vague and doesn't have a clear scope.
        </Reasoning>
        <Verdict>No</Verdict>
    </Example>
    <Example>
        <Prompt>
            Migrate `logger.js` from JavaScript to TypeScript. We already have a `tsconfig.json` and a `LoggerTest.test.js` suite for validation. Make sure it compiles without errors and make sure not to change the existing config!
        </Prompt>
        <Reasoning>
            There’s a clear template (tsconfig.json) and test suite for immediate feedback.
        </Reasoning>
        <Verdict>Yes</Verdict>
    </Example>
    <Example>
        <Prompt>
            Build exactly what’s shown in this Figma mock
        </Prompt>
        <Reasoning>
            This is a subjective visual request. We can’t "see" how humans do and won’t get the details perfectly.
        </Reasoning>
        <Verdict>No</Verdict>
    </Example>
    <Example>
        <Prompt>
            We’re switching from `pg` to `sequelize` (read <https://sequelize.org/api/v6/identifiers>). Please update the `UserModel` queries to use Sequelize methods. Refer to `OrderModel` for how we do it in this codebase.
        </Prompt>
        <Reasoning>
            We can mimic a known pattern and there are explicit references (`OrderModel.js`). Provides a link to docs so we know to reference them.
        </Reasoning>
        <Verdict>Yes</Verdict>
    </Example>
    <Example>
        <Prompt>
            Build a new microservices architecture for our app.
        </Prompt>
        <Reasoning>
            This is a very large and unstructured task. Requires architectural decisions and trade-offs.
        </Reasoning>
        <Verdict>No</Verdict>
    </Example>
</Examples>

<Prompt>
    Create a new endpoint `/users/stats` that returns a JSON object with user count and average signup age. Use our existing users table in PostgreSQL. You can reference the `/orders/stats` endpoint in `statsController.js` for how we structure responses. Ensure the new endpoint is covered by the `StatsController.test.js` suite.
</Prompt>
<Reasoning>
    There’s a clear template (`/orders/stats`), references to guide (`statsController.js`), and a well-defined scope.
</Reasoning>
<Verdict>Yes</Verdict>
