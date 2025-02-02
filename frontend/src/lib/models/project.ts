import Github from 'lucide-svelte/icons/github';
import Gitlab from 'lucide-svelte/icons/gitlab';
import HardDrive from 'lucide-svelte/icons/hard-drive';
import FileQuestion from 'lucide-svelte/icons/file-question';

export interface Project {
    id: string;
    name: string;
    url?: string;
    platform: string;
    icon: any;
}

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
