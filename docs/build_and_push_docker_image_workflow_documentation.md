# Build and Push Docker Image Workflow Documentation

This document provides an overview and detailed explanation of the GitHub Actions workflow that automates the building and pushing of Docker images to the GitHub Container Registry (GHCR).

## Overview

The **Build and Push Docker Image** workflow is designed to:

- **Automate Docker Image Builds**: Whenever a new release is published or the workflow is manually triggered, the workflow builds a Docker image from your repository's code.
- **Push to GitHub Container Registry**: The built Docker image is then pushed to GHCR, tagged appropriately for easy versioning and retrieval.
- **Support Multiple Platforms**: The workflow is set up to build images for multiple platforms, ensuring broad compatibility.

## Workflow Triggers

The workflow is triggered in two scenarios:

1. **On Release Published**: Automatically runs when a new release is published in the repository.
2. **Manual Trigger**: Can be manually triggered via the GitHub Actions tab, allowing you to specify a custom release tag.

## Workflow Details

### Name

- **Workflow Name**: `Build and Push Docker Image`

### Trigger Events

```yaml
on:
  release:
    types: [published]
  workflow_dispatch:
    inputs:
      release_tag:
        description: "Release tag to build and push"
        required: true
```

- **Release Published**: Listens for the `published` event on releases.
- **Workflow Dispatch**: Allows manual triggering with an input `release_tag`.

### Permissions

```yaml
permissions:
  contents: read    # Allows read access to the repository code
  packages: write   # Allows pushing images to GitHub Container Registry
```

- **Contents**: Read access to fetch the repository code.
- **Packages**: Write access to push Docker images to GHCR.

## Job Breakdown

### Job: `build-and-push`

#### Runs On

- **Environment**: `ubuntu-latest`

#### Steps

1. **Checkout Code**

   ```yaml
   - name: Checkout code
     uses: actions/checkout@v4
   ```

   - **Description**: Checks out the repository code so that the workflow can access it.

2. **Set Up Docker Buildx**

   ```yaml
   - name: Set up Docker Buildx
     uses: docker/setup-buildx-action@v3
   ```

   - **Description**: Sets up Docker Buildx for building multi-platform images.

3. **Log in to GitHub Container Registry**

   ```yaml
   - name: Log in to GitHub Container Registry
     uses: docker/login-action@v3
     with:
       registry: ghcr.io
       username: ${{ github.actor }}
       password: ${{ secrets.GITHUB_TOKEN }}
   ```

   - **Description**: Authenticates with GHCR using your GitHub credentials.

4. **Determine the Release Tag**

   ```yaml
   - name: Determine the release tag
     id: get_tag
     run: |
       if [ "${{ github.event_name }}" == "release" ]; then
         echo "tag=${GITHUB_REF#refs/tags/}" >> $GITHUB_OUTPUT
       else
         echo "tag=${{ github.event.inputs.release_tag }}" >> $GITHUB_OUTPUT
       fi
   ```

   - **Description**: Determines the Docker image tag:
     - If triggered by a release, uses the release tag.
     - If manually triggered, uses the provided `release_tag` input.

5. **Build and Push Docker Image**

   ```yaml
   - name: Build and push Docker image
     uses: docker/build-push-action@v6
     with:
       context: .
       push: true
       tags: ghcr.io/${{ github.repository_owner }}/${{ github.event.repository.name }}:${{ steps.get_tag.outputs.tag }}
       platforms: linux/arm64,linux/amd64
   ```

   - **Description**: Builds and pushes the Docker image to GHCR for the specified platforms.

## Platforms

The workflow builds Docker images for the following platforms:

- **linux/arm64**
- **linux/amd64**

**Note**: Initially, the aim was to build for additional platforms such as `darwin/amd64` and `windows/amd64`, but due to local limitations, the workflow currently builds for the platforms listed above.

## Requirements and Prerequisites

To effectively use this workflow, ensure the following:

- **Dockerfile**: A valid `Dockerfile` must be present at the root of your repository.
- **GitHub Permissions**: The `GITHUB_TOKEN` provided by GitHub Actions must have the necessary permissions (default settings typically suffice).
- **Understanding of Docker and GHCR**: Basic knowledge of Docker image building and pushing to GHCR will be beneficial.

## How to Use the Workflow

### Triggering the Workflow on Release

1. **Create a New Release**:

   - Navigate to the "Releases" section of your repository.
   - Click on "Draft a new release".
   - Fill in the release details and publish it.

2. **Workflow Execution**:

   - Upon publishing, the workflow automatically triggers.
   - It will build the Docker image using the release tag and push it to GHCR.

### Manually Triggering the Workflow

1. **Navigate to GitHub Actions**:

   - Go to the "Actions" tab in your repository.

2. **Select the Workflow**:

   - Find the **Build and Push Docker Image** workflow from the list.

3. **Run the Workflow**:

   - Click on the "Run workflow" button.
   - Provide the required `release_tag` input when prompted.
   - Confirm to start the workflow.

4. **Workflow Execution**:

   - The workflow uses the provided `release_tag` to build and push the Docker image.

## Accessing the Pushed Docker Image

The Docker image will be available at:

```
ghcr.io/<repository_owner>/<repository_name>:<tag>
```

Replace:

- `<repository_owner>`: Your GitHub username or organization name.
- `<repository_name>`: The name of your repository.
- `<tag>`: The release tag used during the build.

## Additional Notes

- **Customizing Platforms**: If you need to build for additional platforms, you can modify the `platforms` parameter in the workflow. Be cautious of compatibility and build environment limitations.
- **Troubleshooting**:

  - **Build Failures**: Check the workflow logs for detailed error messages.
  - **Authentication Issues**: Ensure that `GITHUB_TOKEN` has the necessary permissions.
  - **Dockerfile Errors**: Verify that your `Dockerfile` is correctly set up for multi-platform builds.

- **Security Considerations**:

  - The `GITHUB_TOKEN` is scoped to the repository and should be kept secure.
  - Avoid echoing sensitive information in the workflow logs.

## Understanding the Workflow's Impact

By integrating this workflow:

- **Consistency**: Ensures that Docker images are consistently built and tagged with each release.
- **Automation**: Reduces manual efforts in building and pushing Docker images.
- **Version Control**: Leveraging release tags helps in maintaining versions of your Docker images aligned with your codebase.

## Conclusion

This workflow enhances your CI/CD pipeline by automating the Docker image build and push process. It is essential to familiarize yourself with its steps and requirements to leverage its full potential effectively. Should you have any questions or need further assistance, feel free to reach out or consult the GitHub Actions and Docker documentation.

---

*This documentation aims to provide a comprehensive understanding of the new workflow and the requirements it places on you. Ensure to review and customize any parts of the workflow to suit your specific needs.*