# Unanswered Issue Reminder

[Deploy this bot on your GitHub repo via flows.network](#deploy-your-own-unanswered-issues-alert-bot), and you will get a Slack bot that reminds you when an issue raised to your GitHub repo hasn't been responded to in a certain number of days. You can set how many days freely. It helps DevRel and maintainers to track the progress of GitHub issues.

<img width="435" alt="image" src="https://github.com/flows-network/follow-up-github-issue/assets/45785633/60725355-b199-4171-95dd-8f3e6c3a9417">


This is a simple example of how flows.network works: When the GitHub issue doesn't get any response in 2 days, you will receive a Slack message.

## Deploy your own unanswered issues alert bot

1. Load the [unanswered GitHub issues reminder](https://flows.network/flow/createByTemplate/unanswered-issue-reminder) template.
2. Configure and authorize your Slack access
3. Configure the bot to access a specified GitHub repo

### 0 Prerequisites

You will need to sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

### 1 Load the template for monitoring the unanswered GitHub issues

[**Just click here**](https://flows.network/flow/createByTemplate/unanswered-issue-reminder)

[<img width="450" alt="image" src="https://github.com/flows-network/follow-up-github-issue/assets/45785633/0768399f-8236-45d1-868b-2128378a8a58">](https://github.com/flows-network/follow-up-github-issue/assets/45785633/0768399f-8236-45d1-868b-2128378a8a58)


Review the `n_days` variable. This number decides how long to trigger this flow. The number here must be a positive integer greater than or equal to 1. 

Click on the **Create and Build** button.

### 2 Configure the bot to access your Slack

Next, you will tell the bot which Slack channel you want your alert messages to be sent to.

* `slack_channel`:
enter the name of your Slack organization where you want to deploy the bot. Case sensitive.

* `slack_workspace`:
enter the name of the Slack channel where you want to deploy the bot. Case sensitive.

Enter your Slack workspace and channel respectively in the red boxes below.

[<img width="450" alt="image" src="https://github.com/flows-network/github-star-slack-messenger/assets/37167103/7ec3b6ce-c180-4fec-8546-2dddfb9f3d85">](https://github.com/flows-network/github-star-slack-messenger/assets/37167103/7ec3b6ce-c180-4fec-8546-2dddfb9f3d85)


Next, let's grant flows.network to access the Slack channel you just entered. The Slack channel must be public.

Click the "Connect/+ Add new authentication" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network` bot on your workspace. This workspace is the one you entered into the `slack_workspace` above.

> If you have authenticated the Slack access before, you can see the purple Connect button turns gray Connected button. Just ignore this step and continue the next steps.

### 3 Configure the bot to access GitHub

Next, you will tell the bot which GitHub repo's issues need to monitor.

* `github_owner`: enter the name of the GitHub org for the repo *you want to deploy the 🤖 on*.
* `github_repo` : enter the name of GitHub repo *you want to deploy the 🤖 on*.

Click on the **Connect** or **+ Add new authentication** button to give the function access to the GitHub repo to deploy the 🤖. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to the repo.

[<img width="450" alt="image" src="https://github.com/flows-network/github-fork-alert-slack/assets/45785633/869e4a84-76b4-4613-b8e3-d75b94c82f50">](https://github.com/flows-network/github-fork-alert-slack/assets/45785633/869e4a84-76b4-4613-b8e3-d75b94c82f50)


> If you have authenticated the GitHub access before, you can see the purple Connect button turns gray Connected button. Just ignore this step and click the Check button.

### 4. Click the Deploy button to deploy your function.

After that, click the Check button to see your flow details. As soon as the flow function's status becomes `ready` and the flow's status became `running`, you will get a Slack message every time the GitHub issue isn't responded to in time.




