import './assets/index.ts';
import('./pkg').then((pkg: { run_app: () => void }) => pkg.run_app());
