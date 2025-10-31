    // app.module.ts
    import { bootstrapApplication } from '@angular/platform-browser';
    import { PortalComponent } from './components/portal/portal.component';
    import { importProvidersFrom } from '@angular/core';
    import { TextFieldModule } from '@angular/cdk/text-field';

    bootstrapApplication(PortalComponent, {
      providers: [
        importProvidersFrom(TextFieldModule)
      ]
    });