---
nav: Components
group: Application
title: Claude Code
atomId: ClaudeCode
description: https://code.claude.com
---

## Icons

```tsx
import { ClaudeCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <ClaudeCode size={64} />
    <ClaudeCode.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { ClaudeCode } from '@lobehub/icons';

export default () => <ClaudeCode.Text size={48} />;
```

## Combine

```tsx
import { ClaudeCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <ClaudeCode.Combine size={64} />
    <ClaudeCode.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { ClaudeCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <ClaudeCode.Avatar size={64} />
    <ClaudeCode.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { ClaudeCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={ClaudeCode.colorPrimary} />
  </Flexbox>
);
```
