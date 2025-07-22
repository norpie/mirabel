import Github from 'lucide-svelte/icons/github';
import Gitlab from 'lucide-svelte/icons/gitlab';
import HardDrive from 'lucide-svelte/icons/hard-drive';
import FileQuestion from 'lucide-svelte/icons/file-question';
import type { Workspace as GeneratedWorkspace, WorkspaceRole } from '../generated';

export type Workspace = GeneratedWorkspace & {
    logo?: any;
};

export type { WorkspaceRole };

function platformLogo(name: string): any {
    console.log(name);
    switch (name) {
        case 'GitHub':
            return Github;
        case 'GitLab':
            return Gitlab;
        case 'Local':
            return HardDrive;
        default:
            return FileQuestion;
    }
}
