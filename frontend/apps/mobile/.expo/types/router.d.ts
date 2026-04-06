/* eslint-disable */
import * as Router from 'expo-router';

export * from 'expo-router';

declare module 'expo-router' {
  export namespace ExpoRouter {
    export interface __routes<T extends string | object = string> {
      hrefInputParams: { pathname: Router.RelativePathString, params?: Router.UnknownInputParams } | { pathname: Router.ExternalPathString, params?: Router.UnknownInputParams } | { pathname: `/`; params?: Router.UnknownInputParams; } | { pathname: `/_sitemap`; params?: Router.UnknownInputParams; } | { pathname: `${'/(auth)'}/register-maker-confirmation` | `/register-maker-confirmation`; params?: Router.UnknownInputParams; } | { pathname: `${'/(auth)'}/register-maker` | `/register-maker`; params?: Router.UnknownInputParams; };
      hrefOutputParams: { pathname: Router.RelativePathString, params?: Router.UnknownOutputParams } | { pathname: Router.ExternalPathString, params?: Router.UnknownOutputParams } | { pathname: `/`; params?: Router.UnknownOutputParams; } | { pathname: `/_sitemap`; params?: Router.UnknownOutputParams; } | { pathname: `${'/(auth)'}/register-maker-confirmation` | `/register-maker-confirmation`; params?: Router.UnknownOutputParams; } | { pathname: `${'/(auth)'}/register-maker` | `/register-maker`; params?: Router.UnknownOutputParams; };
      href: Router.RelativePathString | Router.ExternalPathString | `/${`?${string}` | `#${string}` | ''}` | `/_sitemap${`?${string}` | `#${string}` | ''}` | `${'/(auth)'}/register-maker-confirmation${`?${string}` | `#${string}` | ''}` | `/register-maker-confirmation${`?${string}` | `#${string}` | ''}` | `${'/(auth)'}/register-maker${`?${string}` | `#${string}` | ''}` | `/register-maker${`?${string}` | `#${string}` | ''}` | { pathname: Router.RelativePathString, params?: Router.UnknownInputParams } | { pathname: Router.ExternalPathString, params?: Router.UnknownInputParams } | { pathname: `/`; params?: Router.UnknownInputParams; } | { pathname: `/_sitemap`; params?: Router.UnknownInputParams; } | { pathname: `${'/(auth)'}/register-maker-confirmation` | `/register-maker-confirmation`; params?: Router.UnknownInputParams; } | { pathname: `${'/(auth)'}/register-maker` | `/register-maker`; params?: Router.UnknownInputParams; };
    }
  }
}
