import MDEditor from '@uiw/react-md-editor';

interface CardContentEditorProps {
  content: string;
  setContent: (content: string | undefined) => void;
}

export default function CardContentEditor({ content, setContent }: CardContentEditorProps) {
  if (content) {
    return <MDEditor value={content} onChange={setContent} height="750px" />;
  } else {
    return '';
  }
}
